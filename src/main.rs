use clap::{Arg, ArgAction, Command};
use neomanager::handlers;

fn main() {
    let mut command = Command::new("neomanager")
        .version("0.2.0")
        .about("Manage neovim configs")
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .help("Activate verbose mode")
                .action(ArgAction::SetTrue),
        )
        .subcommand(Command::new("backup").about("Make neovim's config backup"))
        .subcommand(Command::new("restore").about("Restore neovim's config from backup"));

    let matches = command.clone().get_matches();
    match matches.subcommand() {
        Some(("backup", sub_matches)) => handlers::backup::run_backup(&matches, sub_matches),
        Some(("restore", sub_matches)) => handlers::restore::run_restore(&matches, sub_matches),
        None => {
            command.print_help().unwrap();
            println!();
        }
        _ => unreachable!(),
    }
}
