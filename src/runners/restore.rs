use crate::utils::fs;
use std::env;
use std::path::PathBuf;

pub struct RestoreRunner {}

impl RestoreRunner {
    pub fn new() -> Self {
        RestoreRunner {}
    }

    pub fn exec(&self) {
        let home = PathBuf::from(env::var("HOME").expect("HOME environment variable is not set."));
        let config_folder = home.join(".config/nvim");

        // find last backup folder
        let last_backup = fs::find_last_backup().expect("Backup not found.");

        // delete config folder
        if config_folder.is_dir() {
            std::fs::remove_dir_all(config_folder.clone())
                .expect("Error deleting the current config.");
        }

        // copy last backup
        fs::copy_directory(last_backup, config_folder.clone()).expect("Error copying backup.");
    }
}
