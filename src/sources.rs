use std::collections::HashMap;
use std::fs;
use std::io::Write;

use serde_derive;
use serde_json;
use failure::Error;
use failure::format_err;

pub fn load_sources() -> Result<Sources, Error> {
    println!("Loading sources...");
    let sources_file = fs::read_to_string("sources.json")?;
    Sources::new(&sources_file)
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Sources {
    pub sources: HashMap<String, Source>,
}

impl Sources {
    pub fn new(sources_file: &str) -> Result<Sources, Error> {
        let json: Sources = serde_json::from_str(sources_file)?;
        // let data: HashMap<String, Source> = json.sources;
        Ok(Sources { sources: json.sources })
    }

    // pub fn load(&mut self) -> Result<Sources, Error> {
    //     println!("Loading sources...");
    //     let sources_file = fs::read_to_string("sources.json")?;
    //     self.sources = serde_json::from_str(&sources_file)?;
    //     Ok(*self)
    // }

    pub fn save(&self) -> Result<(), Error> {
        println!("Saving sources...");
        let output = serde_json::to_string(self)?;
        let output_bytes = output.as_bytes();
        let mut sources_file = fs::File::create("sources.json")?;
        
        match sources_file.write(output_bytes) {
            Ok(_n) => Ok(()),
            _ => Err(format_err!("Problem writing sources file"))
        }
        
    }

    pub fn list(&self) -> Result<Vec<String>, Error> {
        let mut repos: Vec<String> = Vec::new();
        self.sources.keys().for_each(|repo| repos.push(repo.to_string()));
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

    pub fn update_latest_release(&mut self, repo: &str, release: ReleaseInfo) -> Result<(), Error> {
        match self.sources.get_mut(repo) {
            Some(x) => {
                x.update_latest_release(release);
                Ok(())
            },
            None => {
                Err(format_err!("Source not found"))
            }
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Source {
    // pub repository: String,
    pub install_command: String,
    pub update_command: String,
    pub latest_release: ReleaseInfo,
}

impl Source {
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