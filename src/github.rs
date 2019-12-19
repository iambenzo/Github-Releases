use reqwest;
mod gh_release;

pub fn get_download_url(repo: &str) -> Result<String, reqwest::Error> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    // println!("Request URL: {}", url);

    let mut response = reqwest::get(&url)?;

    let latest_release: gh_release::Release = response.json()?;
    // println!("{:?}", latest_release);

    let download_url = &latest_release.zipball_url;
    // println!("Download URL: {}", download_url);

    Ok(download_url.to_string())
}