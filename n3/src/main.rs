mod args;
mod exec;
mod monitor;

use clap::{crate_authors, crate_version, App, AppSettings, Arg, ArgMatches, Result as ClapResult};
use inflector::Inflector;

use n3_builder::{ast, dirs, inflector, ExecRoot, GlobalVars, Result, Vars, QUERY_SPLIT_1};

use crate::args::Command;

pub const SWITCH_FN_1: &[(&str, FnExec)] = &[];
pub const SWITCH_FN_2: &[&str] = &["train", "eval", "publish"];

pub type FnExec = fn(Command) -> Result<()>;

fn main() -> Result<()> {
   // Step 1. parse envs
   let env = GlobalVars::default();
   let env_vars = env.to_n3_variables();

   // Step 2. parse command & exec
   let cmd_args: Vec<_> = std::env::args().skip(1).collect();
   let command = cmd_args.get(0);
   let exec = cmd_args.get(1);

   // Step 3-1. execute commands that don't need a root
   if let Some((_, f)) = command
      .map(|x| SWITCH_FN_1.iter().find(|(k, _)| k == x))
      .flatten()
   {
      let args_set = [&env_vars];
      match unsafe { parse_args(&args_set) } {
         Ok(_) => {
            f(Command {
               command: command.unwrap(),
               env: &env,
               args: None,
            })?;
         }
         Err(e) => {
            println!("{}", e);
         }
      }

      // drop order: app (=matches) -> args
      drop(env);
      Ok(())
   }
   // Step 3-2. execute commands with a root
   else if let Some(exec) = exec {
      let mut root = ExecRoot::try_new(env.clone())?;
      let args = root.get(&exec.to_pascal_case())?;

      let args_set = [&env_vars, &args.to_exec_variables()];
      match unsafe { parse_args(&args_set) } {
         Ok(_) => {
            crate::exec::execute(Command {
               command: command.unwrap(),
               env: &env,
               args: Some(args),
            })?;
         }
         Err(e) => {
            println!("{}", e);
         }
      }

      // drop order: app (=matches) -> args
      drop(root);
      drop(env);
      Ok(())
   }
   // Step 3-3. show help message
   else {
      let app = unsafe { subcommand_args(&env_vars, app()) };
      let matches = app.get_matches_from(&["--help"]);

      // drop order: app (=matches) -> args
      drop(matches);
      drop(env);
      Ok(())
   }
}

unsafe fn parse_args<'a, 'b, 'c>(args: &[&'a Vars]) -> ClapResult<Result<ArgMatches<'b>>>
where
   'a: 'b,
   'b: 'c,
{
   let mut app = app();
   for args in args {
      app = subcommand_args(args, app);
   }

   let matches = app.get_matches_safe()?;
   for args in args {
      if let Err(e) = apply(&matches, args) {
         return Ok(Err(e));
      }
   }
   Ok(Ok(matches))
}

fn apply(matches: &ArgMatches, args: &Vars) -> Result<()> {
   for name in args.inner.keys() {
      if let Some(values) = matches.values_of(name) {
         let value = values.collect::<Vec<_>>().join(QUERY_SPLIT_1);
         args.set(name, &value)?;
      }
   }
   Ok(())
}

unsafe fn subcommand_args<'a, 'b, 'c>(args: &'a Vars, mut app: App<'b, 'c>) -> App<'b, 'c>
where
   'a: 'b,
   'b: 'c,
{
   for (name, var) in &args.inner {
      // drop order: app -> args
      let var = var.try_borrow_unguarded().unwrap();

      // hidden
      if var.name == dirs::N3_SOURCE_ROOT {
         continue;
      }

      let mut arg = Arg::with_name(name).long(name).takes_value(true);
      if let Some(shortcut) = &var.shortcut {
         arg = arg.short(shortcut.to_lowercase());
      }
      if let Some(desc) = &var.description {
         arg = arg.help(desc);
      }
      if let Some(ast::LetType::List(_)) = var.ty {
         arg = arg.multiple(true);
      }
      if var.value.is_none() {
         arg = arg.required(true);
      }

      app = app.arg(arg);
   }
   app
}

fn app<'a, 'b, 'c>() -> App<'b, 'c>
where
   'a: 'b,
   'b: 'c,
{
   let exec_commands: Vec<_> = SWITCH_FN_2.iter().map(|x| ("command", *x)).collect();

   App::new("n3")
      .version(crate_version!())
      .author(crate_authors!())
      .about("Neural Network Notation")
      .arg(
         Arg::with_name("root_dir")
            .long("root_dir")
            .help("The n3 program's own local directory")
            .takes_value(true),
      )
      .setting(AppSettings::ArgRequiredElseHelp)
      .setting(AppSettings::ColoredHelp)
      .setting(AppSettings::ColorAuto)
      .setting(AppSettings::GlobalVersion)
      .arg(Arg::with_name("command").required(true))
      .arg(Arg::with_name("exec").required_ifs(&exec_commands))
}
