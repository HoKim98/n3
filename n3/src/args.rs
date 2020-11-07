use clap::{crate_authors, crate_version, App, Arg};

use n3_builder::{ast, GlobalVars, RawVariables};

pub type FnExec = fn(GlobalVars, &clap::ArgMatches);

struct CommandArgs {
    env: GlobalVars,
    vars: RawVariables,
}

impl Default for CommandArgs {
    fn default() -> Self {
        let env = GlobalVars::default();
        let vars = env.to_n3_variables();
        Self { env, vars }
    }
}

pub fn get_matches(f: FnExec) {
    let args = CommandArgs::default();
    let matches = app(&args.vars).get_matches();
    f(args.env, &matches)
}

fn subcommand_env<'a, 'b, 'c>(env: &'b RawVariables, mut app: App<'b, 'c>) -> App<'b, 'c>
where
    'a: 'b,
    'b: 'c,
{
    for (name, var) in env {
        let mut arg = Arg::with_name(name).long(name).takes_value(true);
        if let Some(desc) = &var.description {
            arg = arg.help(desc);
        }
        if let Some(ast::LetType::List(_)) = var.ty {
            arg = arg.multiple(true);
        }

        app = app.arg(arg);
    }
    app
}

fn subcommand<'a, 'b, 'c>(env: &'b RawVariables, name: &str, about: &'c str) -> App<'b, 'c>
where
    'a: 'b,
    'b: 'c,
{
    subcommand_env(
        env,
        App::new(name).about(about).arg(
            Arg::with_name("exec")
                .help("The execution file's name")
                .required(true),
        ),
    )
}

fn app<'a, 'b, 'c>(env: &'b RawVariables) -> App<'b, 'c>
where
    'a: 'b,
    'b: 'c,
{
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
        .subcommand(subcommand(
            env,
            "train",
            "Train the model on the specific execution",
        ))
        .subcommand(subcommand(
            env,
            "eval",
            "Evaluate the model on the specific execution",
        ))
        .subcommand(subcommand(
            env,
            "publish",
            "Publish the model on the specific execution",
        ))
        .subcommand(subcommand_env(
            env,
            App::new("monitor").about("Open the Tensorboard"),
        ))
}
