use std::env;
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

pub fn find_last_backup() -> Result<PathBuf, io::Error> {
    let home = PathBuf::from(env::var("HOME").expect("HOME environment variable is not set."));
    let backup_folder = home.join(".config/neomanager/backups");

    if !backup_folder.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "No backup folder founded.",
        ));
    }

    let mut dirs = Vec::new();

    for entry in fs::read_dir(backup_folder)? {
        let entry = entry?;

        if entry.file_type()?.is_dir() {
            dirs.push(entry.path());
        }
    }

    dirs.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

    match dirs.last() {
        Some(dir) => Ok(dir.to_path_buf()),
        None => {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "No backup founded.",
            ));
        }
    }
}
