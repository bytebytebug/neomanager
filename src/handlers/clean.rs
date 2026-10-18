use crate::runners::clean;
use clap::{ArgMatches};

pub fn run_clean(_root_matches: &ArgMatches, _sub_matches: &ArgMatches) {
    let runner = clean::CleanRunner::new();

    runner.exec();
}
