use reqwest;
mod gh_release;

pub fn get_release_info(repo: &str) -> Result<gh_release::Release, reqwest::Error> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    // println!("Request URL: {}", url);

    let mut response = reqwest::get(&url)?;

    let latest_release: gh_release::Release = response.json()?;
    Ok(latest_release)
}
