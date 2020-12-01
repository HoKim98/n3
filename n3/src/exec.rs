use n3_builder::{dirs, Result};
use n3_net_client::Work;

use crate::Command;

pub fn execute(command: Command) -> Result<()> {
    let machines = command.env.get_string_list(dirs::N3_MACHINES)?;

    // Step 1. build a program
    let program = command.args.unwrap().build_with_env()?;

    // Step 2. spawn a work
    let work = Work::spawn(&program, command.command, &machines).unwrap();

    // Step 3. wait the work
    drop(work);

    // Step 4. finalize
    Ok(())
}
