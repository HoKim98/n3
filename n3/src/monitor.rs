use std::process;

use n3_builder::{dirs, Result};

use crate::Command;

pub const PORT: u16 = 40962;

pub fn f(command: Command) -> Result<()> {
    let root = command.env.get_string(dirs::N3_ROOT)?;

    let mut child = process::Command::new("tensorboard")
        .arg("--port")
        .arg(PORT.to_string())
        .arg("--logdir")
        .arg(format!("{}/{}", root, dirs::LOGS_DIR))
        // .arg("--bind_all")
        .spawn()
        .expect("failed to execute process");

    let ecode = child.wait().expect("failed to wait on child");
    let code = ecode.code().unwrap_or_default();

    process::exit(code);
}
