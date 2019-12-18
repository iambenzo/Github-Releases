use std::cmp::min;
use std::collections::HashMap;
use std::io::Error;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::Read;
use std::io::copy;


use indicatif::{ProgressBar, ProgressStyle};
// use reqwest::{Client, Url, UrlError};
// use reqwest::header::{ContentType,ContentLength};
use mrq;

type Headers = HashMap<String, String>;

fn get_headers(url: &str) -> Result<Headers, Error> {
    let req = mrq::head(url)
        .with_header("Accept", "*/*")
        .with_header("User-Agent", "ua")
        .with_timeout(30)
        .send()?;

    Ok(req.headers)
}

fn create_progress_bar(quiet_mode: bool, msg: &str, length: u64) -> ProgressBar {
    let bar = match quiet_mode {
        true => ProgressBar::hidden(),
        false => match length > 0 {
            true => ProgressBar::new(length),
            false => ProgressBar::new_spinner(),
        },
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

fn save_to_file(contents: &mut Vec<u8>, fname: &str) -> Result<(), std::io::Error> {
    let mut file = File::create(fname).unwrap();
    copy(&mut contents.as_slice(), &mut file).unwrap();
    Ok(())

}

pub fn download_file(repo: &str) -> Result<(), Box<std::error::Error>> {
    let mut downloaded = 0;
    // let total_size = 231231231;
    let total_size: u64 = {
        if let Some(size) = get_headers(&repo).unwrap().get("Content-Length") {
            size.parse::<u64>().unwrap_or(0)
        } else {
            0u64
        }
    };

    let chunk_size = match total_size > 0 {
        true => total_size as usize / 99,
        false => 1024usize,
    };

    // let bar = create_progress_bar(false, &repo, total_size);

    let bar = ProgressBar::new(total_size);
    bar.set_message(&repo);
    bar.set_style(ProgressStyle::default_bar().template(
        "{msg:.green} [{elapsed_precise}] {bar:40.cyan/blue} {bytes}/{total_bytes} ({eta})",
    ));

    while downloaded < total_size {
        let new = min(downloaded + chunk_size as u64, total_size);
        downloaded = new;
        bar.set_position(new);
        thread::sleep(Duration::from_millis(12));
    }

    // let mut resp = mrq::get(repo)
    //     .with_header("Accept", "*/*")
    //     .with_header("User-Agent", "ua")
    //     .with_timeout(30)
    //     .send()?;

    // let mut buf = Vec::new();

    // while downloaded < total_size {
        
    // }

    // let mut cnt = 0;
    // loop {
    //     let mut buffer = vec![0; chunk_size];
    //     let bcount = resp.body.read(&mut buffer[..]).unwrap();
    //     cnt += bcount;
    //     buffer.truncate(bcount);
    //     if !buffer.is_empty() {
    //         buf.extend(buffer.into_boxed_slice().into_vec().iter().cloned());
    //         bar.set_position(cnt as u64);
    //     } else {
    //         break;
    //     }
    //     if Some(cnt) == Some(total_size as usize) {
    //         break;
    //     }
    // }

    bar.finish();

    // save_to_file(&mut buf, "test")?;

    Ok(())
}
