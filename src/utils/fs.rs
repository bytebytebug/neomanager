use std::fs;
use std::io;
use std::path::PathBuf;

pub fn copy_directory(source: PathBuf, destination: PathBuf) -> Result<(), io::Error> {
    fs::create_dir_all(destination.clone())?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.clone().join(entry.file_name());

        if source_path.is_dir() {
            copy_directory(source_path, destination_path)?;
        } else {
            fs::copy(&source_path, &destination_path)?;
        }
    }

    Ok(())
}

pub fn create_if_not_exist(folder: PathBuf) -> Result<(), io::Error> {
    if !folder.exists() {
        std::fs::create_dir_all(folder)
    } else {
        Ok(())
    }
}
