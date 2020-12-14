use std::env;
use std::marker::PhantomData;
use std::mem;

use raw_sync::locks::{LockImpl, LockInit, Mutex};
use shared_memory::{Shmem, ShmemConf, ShmemError};

use crate::error::{Result, SMPError};

pub struct SMPool<SM> {
    name: String,
    shmem: mem::ManuallyDrop<Shmem>,
    mutex: mem::ManuallyDrop<Box<dyn LockImpl>>,
    _sm: PhantomData<SM>,
}

impl<SM> Clone for SMPool<SM>
where
    SM: Default,
{
    fn clone(&self) -> Self {
        Self::open(&self.name).unwrap()
    }
}

impl<SM> SMPool<SM>
where
    SM: Default,
{
    pub fn create<S: AsRef<str>>(name: S) -> Result<Self> {
        Self::new(
            &name,
            Self::new_conf(&name).create().map_err(SMPError::from)?,
            true,
        )
    }

    pub fn open<S: AsRef<str>>(name: S) -> Result<Self> {
        Self::new(
            &name,
            Self::new_conf(&name).open().map_err(SMPError::from)?,
            false,
        )
    }

    pub fn create_or_open<S: AsRef<str>>(name: S) -> Result<Self> {
        let (shmem, create) = match Self::new_conf(&name).create() {
            Ok(m) => (m, true),
            Err(ShmemError::LinkExists) => {
                (Self::new_conf(&name).open().map_err(SMPError::from)?, false)
            }
            Err(e) => return Err(SMPError::from(e).into()),
        };

        Self::new(&name, shmem, create)
    }

    fn new_conf<S: AsRef<str>>(name: S) -> ShmemConf {
        let mut dir = env::temp_dir();
        dir.push(format!("n3-{}.lock", name.as_ref()));

        let size = mem::size_of::<Mutex>() + mem::size_of::<SM>();
        ShmemConf::new().size(size).flink(dir)
    }

    fn new<S: AsRef<str>>(name: S, shmem: Shmem, create: bool) -> Result<Self> {
        let base_ptr = shmem.as_ptr();
        let (mutex, _) = unsafe {
            let size = Mutex::size_of(Some(base_ptr));
            let inner_ptr = base_ptr.add(size);
            Mutex::new(base_ptr, inner_ptr).map_err(SMPError)?
        };

        let pool = Self {
            name: name.as_ref().to_string(),
            shmem: mem::ManuallyDrop::new(shmem),
            mutex: mem::ManuallyDrop::new(mutex),
            _sm: Default::default(),
        };
        if create {
            pool.with_inner(|data| *data = Default::default())?;
        }

        Ok(pool)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn with_inner<R>(&self, f: impl Fn(&mut SM) -> R) -> Result<R> {
        let guard = self.mutex.lock().map_err(SMPError)?;
        let inner = unsafe { &mut *(*guard as *mut SM) };

        Ok(f(inner))
    }
}

impl<SM> Drop for SMPool<SM> {
    fn drop(&mut self) {
        // order: mutex -> shmem
        unsafe {
            mem::ManuallyDrop::drop(&mut self.mutex);
            mem::ManuallyDrop::drop(&mut self.shmem);
        }
    }
}
