use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

use n3_machine::{Machine, Query};
use n3_torch_ffi::ProcessMachine;

use crate::process::ProcessMachine as ProcessMachineImpl;

pub trait CandidatesMachine {
    fn get_candidates() -> Vec<Query>;

    unsafe fn try_new(query: &Query) -> Vec<Box<dyn Machine>>
    where
        Self: Sized + 'static,
    {
        ProcessMachineImpl::try_new::<CandidatesMachineImpl<Self>>(query)
    }
}

pub struct CandidatesMachineImpl<C> {
    _trait: PhantomData<C>,
    inner: ProcessMachineImpl,
}

impl<C> ProcessMachine<ProcessMachineImpl> for CandidatesMachineImpl<C>
where
    C: CandidatesMachine,
{
    unsafe fn try_new(inner: ProcessMachineImpl) -> Self {
        Self {
            _trait: PhantomData::default(),
            inner,
        }
    }

    fn verify_query(query: &Query) -> Vec<Query> {
        C::get_candidates()
            .into_iter()
            .filter(|x| query.eq_device(x))
            .collect()
    }
}

impl<C> Deref for CandidatesMachineImpl<C> {
    type Target = ProcessMachineImpl;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<C> DerefMut for CandidatesMachineImpl<C> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
