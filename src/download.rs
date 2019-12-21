use std::io::Read;
use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use failure::format_err;
use failure::Error;
use indicatif::{ProgressBar, ProgressStyle};


fn create_progress_bar(msg: &str, length: u64) -> ProgressBar {
    let bar = match length > 0 {
        true => ProgressBar::new(length),
        false => ProgressBar::new_spinner(),
    };

    bar.set_message(msg);
    match length > 0 {
        true => bar.set_style(ProgressStyle::default_bar().template(
            "{msg:.green} [{elapsed_precise}] {bar:40.cyan/blue} {bytes}/{total_bytes} ({eta})",
        )),
        false => bar.set_style(ProgressStyle::default_spinner()),
    };

    bar
}

pub fn download_file(repo: &str, url: &str, file_name: &str) -> Result<(String, usize), Error> {
    let mut download_location: PathBuf = PathBuf::from(r"./gh-releases/");
    download_location.push(Path::new(repo));

    if !download_location.exists() {
        fs::create_dir_all(&download_location)?;
    }

    download_location.push(file_name);

    // println!("Download Location: {}", download_location.to_str().unwrap());

    if download_location.exists() {
        println!("File already exists, skipping download.");
        return Ok((format!("{}", repo), 0));
    }

    let client = reqwest::Client::new();
    let mut response = client.get(url).send()?;

    if response.status().is_success() {
        let total_size: u64 = response
            .headers()
            .get(reqwest::header::CONTENT_LENGTH)
            .and_then(|l| l.to_str().ok())
            .and_then(|l| l.parse().ok())
            .unwrap_or(0);

        let chunk_size = match total_size > 0u64 {
            true => total_size as usize / 99,
            false => 1024usize,
        };

        let progress_bar = create_progress_bar(&repo, total_size);

        let mut buffer: Vec<u8> = Vec::new();
        let mut downloaded = 0;

        loop {
            let mut small_buffer = vec![0; chunk_size];
            let small_buffer_read = response.read(&mut small_buffer[..])?;
            small_buffer.truncate(small_buffer_read);

            match small_buffer.is_empty() {
                true => break,
                false => {
                    buffer.extend(small_buffer);
                    downloaded += small_buffer_read;
                    progress_bar.set_position(downloaded as u64);
                    // progress_bar.inc(small_buffer_read as u64);
                    // progress_bar.set_message(&format!("{} : {} / {}", repo, downloaded, total_size));
                }
            }
        }

        let mut disk_file = fs::File::create(&download_location)?;
        let size_on_disk = disk_file.write(&buffer)?;

        progress_bar.finish();

        Ok((format!("{}", repo), size_on_disk))
    } else {
        Err(format_err!("No response recieved from server."))
    }
}
