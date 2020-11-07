use clap::ArgMatches;

use n3_builder::{ExecRoot, GlobalVars};

pub fn train(env: GlobalVars, matches: &ArgMatches) {
    dbg!(&env);

    let exec = ExecRoot::try_new(env);
    drop(exec);
}
