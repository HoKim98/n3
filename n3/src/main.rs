mod args;
mod eval;
mod monitor;
mod publish;
mod train;

use self::args::{get_matches, FnExec};

const SWITCH: &[(&str, FnExec)] = &[("train", self::train::train)];

fn main() {
   get_matches(|env, matches| {
      for (name, command) in SWITCH {
         if let Some(matches) = matches.subcommand_matches(name) {
            return command(env, matches);
         }
      }
      unreachable!();
   })
}
