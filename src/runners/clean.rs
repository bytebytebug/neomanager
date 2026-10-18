use std::path;
use std::env;

pub struct CleanRunner {

}

impl CleanRunner {
    pub fn new() -> Self {
        CleanRunner{}
    }

    pub fn exec(&self) {
        // find config folder
        let home = path::PathBuf::from(env::var("HOME").expect("HOME variable not found."));
        let config_folder = home.join(".config/nvim");
        // delete config folder
        //
        if config_folder.is_dir() {
            std::fs::remove_dir_all(config_folder).expect("Error deleting config directory.");
        }
    }
}
