pub fn gen_filename(repo: &str, url: &str) -> String {
    let app_name = repo.split("/").last().unwrap();
    let version = url.split("/").last().unwrap();
    format!("{}-{}.zip", app_name, version)
}