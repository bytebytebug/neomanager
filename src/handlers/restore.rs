use crate::runners::restore;
use clap::ArgMatches;

pub fn run_restore(_root_matches: &ArgMatches, _sub_matches: &ArgMatches) {
    let runner = restore::RestoreRunner::new();

    runner.exec();
}
