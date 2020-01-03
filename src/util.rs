use std::fs::remove_file;
use std::path::{Path, PathBuf};
use crate::config;

pub fn gen_filename(repo: &str, url: &str) -> String {
    let app_name = repo.split("/").last().unwrap();
    let version = url.split("/").last().unwrap();
    format!("{}-{}.zip", app_name, version)
}

pub fn delete_old_release(repo: &str, url: &str) -> Result<(), std::io::Error> {
    let file_name = gen_filename(&repo, &url);
    let config = config::Config::new();
    let mut download_location: PathBuf = PathBuf::from(config.get_release_dir());
    download_location.push(Path::new(repo));
    download_location.push(file_name);
    remove_file(download_location)
}
