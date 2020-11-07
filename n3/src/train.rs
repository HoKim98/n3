use clap::ArgMatches;

use n3_builder::{ExecRoot, GlobalVars};

pub fn train(env: GlobalVars, matches: &ArgMatches) {
    let n3_root = matches.value_of("n3_root");
    dbg!(n3_root);

    let env = GlobalVars::default();
    let exec = ExecRoot::try_new(env);
    drop(exec);
}
