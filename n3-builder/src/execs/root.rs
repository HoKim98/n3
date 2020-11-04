use std::fs;
use std::ops::Deref;
use std::path::Path;

use super::dirs::*;
use super::ir::ExecIR;
use super::program::Program;
use super::var::{GlobalVars, Vars};
use crate::error::{ExecError, Result};
use crate::graph::ToValues;
use crate::n3_std::trim_path;
use crate::nodes::NodeRoot;

use glob::glob;

pub struct ExecRoot {
    node_root: NodeRoot,
    env: GlobalVars,
}

impl ExecRoot {
    pub fn try_new(env: GlobalVars, std_dir: Option<&str>) -> Result<Self> {
        let root = Self {
            node_root: NodeRoot::new(std_dir),
            env,
        };

        root.create_root_dir()?;
        root.load_local_nodes()?;

        Ok(root)
    }

    pub fn get(&mut self, name: &str) -> Result<Args> {
        let ir = self.node_root.get_exec(name)?;
        let args = ir.args();

        Ok(Args {
            root: self,
            ir,
            args,
        })
    }

    pub fn attach_env(&self, program: &mut Program) {
        program.env = Some(self.env.to_values());
    }

    fn create_root_dir(&self) -> Result<()> {
        let path = self.env.root_dir();

        if path.exists() {
            if path.is_dir() {
                Ok(())
            } else {
                ExecError::NotDirectory { path }.into()
            }
        } else {
            Self::make_root_dir(&path)
        }
    }

    fn load_local_nodes(&self) -> Result<()> {
        let path = self.env.root_dir().join(NODES_DIR);
        let path = path.join("**/*.n3").display().to_string();

        for path in glob(&path)? {
            let path = path?;
            let name = trim_path(&path);

            let extern_path = path.with_extension("py");
            if extern_path.exists() {
                let path_str = extern_path.display().to_string();
                self.node_root.add_extern_path(name.clone(), path_str);
            }

            let path_str = path.display().to_string();
            self.node_root.add_source_path(name, path_str);
        }
        Ok(())
    }

    #[cfg(feature = "cli")]
    fn make_root_dir(path: &Path) -> Result<()> {
        if dialoguer::Confirm::new()
            .default(false)
            .with_prompt(format!(
                "It seems that there is no root directory on \"{}\"
- Do you want to create one?",
                path.display()
            ))
            .interact()?
        {
            fs::create_dir_all(path)?;
            for name in &[
                Path::new(DATA_DIR),
                Path::new(LOGS_DIR),
                Path::new(MODELS_DIR),
                Path::new(NODES_DIR),
                &Path::new(NODES_DIR).join(NODES_USER_DIR),
            ] {
                fs::create_dir(path.join(name))?;
            }
            Ok(())
        } else {
            no_such_directory(path)
        }
    }

    #[cfg(not(feature = "cli"))]
    fn make_root_dir(path: &Path) -> Result<()> {
        no_such_directory()
    }
}

fn no_such_directory(path: &Path) -> Result<()> {
    ExecError::NoSuchDirectory {
        path: path.to_path_buf(),
    }
    .into()
}

pub struct Args<'a> {
    root: &'a mut ExecRoot,
    ir: ExecIR,
    args: Vars,
}

impl<'a> Deref for Args<'a> {
    type Target = Vars;

    fn deref(&self) -> &Self::Target {
        &self.args
    }
}

impl<'a> Args<'a> {
    pub fn build_uncompacted(self) -> Result<Program> {
        self.ir.build(&self.root.node_root)
    }

    pub fn build_with_env(self) -> Result<Vec<u8>> {
        let mut program = self.ir.build(&self.root.node_root)?;
        self.root.attach_env(&mut program);
        program.save_to_binary()
    }

    pub fn build(self) -> Result<Vec<u8>> {
        self.build_uncompacted()?.save_to_binary()
    }
}
