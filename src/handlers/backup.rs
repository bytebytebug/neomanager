use crate::runners::backup;
use clap::ArgMatches;

pub fn run_backup(_root_matches: &ArgMatches, _sub_matches: &ArgMatches) {
    let runner = backup::BackupRunner::new();

    runner.exec();
}
