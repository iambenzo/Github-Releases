use std::collections::HashMap;
use std::fs;
use std::io::Write;

use failure::format_err;
use failure::Error;
use serde_derive;
use serde_json;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Sources {
    pub sources: HashMap<String, Source>,
}

impl Sources {
    pub fn new() -> Sources {
        let sources: HashMap<String, Source>;
        let source_json = || -> Result<HashMap<String, Source>, Error> {
            let sources_file = fs::read_to_string("sources.json")?;
            let json: Sources = serde_json::from_str(&sources_file)?;
            Ok(json.sources)
        };

        if let Err(_e) = source_json() {
            sources = HashMap::new();
        } else {
            sources = source_json().unwrap();
        }

        Sources { sources }
    }

    pub fn save(&self) -> Result<(), Error> {
        let output = serde_json::to_string(self)?;
        let output_bytes = output.as_bytes();
        let mut sources_file = fs::File::create("sources.json")?;

        match sources_file.write(output_bytes) {
            Ok(_n) => Ok(()),
            _ => Err(format_err!("Problem writing sources file")),
        }
    }

    pub fn list(&mut self) -> Result<Vec<String>, Error> {
        let mut repos: Vec<String> = Vec::new();
        self.sources
            .keys()
            .for_each(|repo| repos.push(repo.to_string()));
        Ok(repos)
    }

    pub fn add_source(&mut self, repo: &str, source: Source) -> Result<(), Error> {
        match self.sources.contains_key(repo) {
            true => Err(format_err!("Source already exists")),
            false => {
                self.sources.insert(repo.to_string(), source);
                Ok(())
            }
        }
    }

    pub fn contains(&mut self, repo: &str) -> bool {
        self.sources.contains_key(repo)
    }

    pub fn update_latest_release(&mut self, repo: &str, release: ReleaseInfo) -> Result<(), Error> {
        match self.sources.get_mut(repo) {
            Some(x) => {
                x.update_latest_release(release);
                Ok(())
            }
            None => Err(format_err!("Source not found")),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Source {
    pub install_command: String,
    pub update_command: String,
    pub latest_release: ReleaseInfo,
}

impl Source {
    pub fn new(
        install_command: String,
        update_command: String,
        latest_release: ReleaseInfo,
    ) -> Source {
        Source {
            install_command,
            update_command,
            latest_release,
        }
    }

    pub fn update_latest_release(&mut self, release: ReleaseInfo) {
        self.latest_release = release;
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ReleaseInfo {
    pub tag_name: String,
    pub name: String,
    pub created_at: String,
    pub published_at: String,
    pub tarball_url: String,
    pub zipball_url: String,
}

impl ReleaseInfo {
    pub fn new(
        tag_name: &str,
        name: &str,
        created_at: &str,
        published_at: &str,
        tarball_url: &str,
        zipball_url: &str,
    ) -> ReleaseInfo {
        ReleaseInfo {
            tag_name: tag_name.to_string(),
            name: name.to_string(),
            created_at: created_at.to_string(),
            published_at: published_at.to_string(),
            tarball_url: tarball_url.to_string(),
            zipball_url: zipball_url.to_string(),
        }
    }
}
