use std::fs::File;
use std::io::copy;
use std::io::Read;

use failure::format_err;
use failure::Error;

use indicatif::{ProgressBar, ProgressStyle};

use std::fs;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

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

// fn parse_url(url: &str) -> Result<Url, UrlError> {
//     match Url::parse(url) {
//         Ok(url) => Ok(url),
//         Err(error) if error == UrlError::RelativeUrlWithoutBase => {
//             let url_with_base = format!("{}{}", "http://", url);
//             Url::parse(url_with_base.as_str())
//         }
//         Err(error) => Err(error),
//     }
// }

fn save_to_file(contents: &mut Vec<u8>, fname: &str) -> Result<(), Error> {
    let mut file = File::create(fname).unwrap();
    copy(&mut contents.as_slice(), &mut file).unwrap();
    Ok(())
}

pub fn download_file(repo: &str) -> Result<(String, usize), Error> {
    let mut download_location: PathBuf = PathBuf::from("C:\\Users\\benbu\\Documents\\Git\\ghr");
    download_location.push(repo);

    if download_location.exists() {
        println!("File already exists, skipping download.");
        return Ok((format!("{}", repo), 0));
    }

    let client = reqwest::Client::new();
    let mut response = client.get(repo).send()?;

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

        // let mut disk_file = fs::File::create(&download_location)?;
        // let size_on_disk = disk_file.write(&buffer)?;

        save_to_file(&mut buffer, "test.zip")?;

        progress_bar.finish();

        // Ok((format!("{}", repo), size_on_disk))
        Ok((format!("{}", repo), downloaded))
    } else {
        Err(format_err!("No response recieved from server."))
    }
}
