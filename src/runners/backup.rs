use crate::utils::fs;
use crate::utils::string;
use std::env;
use std::path::PathBuf;

pub struct BackupRunner {}

impl BackupRunner {
    pub fn new() -> Self {
        Self {}
    }

    pub fn exec(&self) {
        let home = PathBuf::from(env::var("HOME").expect("HOME variable undefined."));
        let config_folder = home.join(".config/nvim");
        let backup_name = string::timestamp();
        let backup_folder = home.join(".config/neomanager/backups");

        fs::create_if_not_exist(backup_folder.clone()).expect("Error creating backup directory.");

        let destination_folder = backup_folder.clone().join(backup_name.clone());
        fs::copy_directory(config_folder.clone(), destination_folder.clone())
            .expect("Error creating backup.");
    }
}
