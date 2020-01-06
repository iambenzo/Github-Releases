use std::collections::HashMap;
use std::fs;
use std::io::Write;

use dirs;
use failure::format_err;
use failure::Error;
use serde_derive;
use serde_json;

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Config {
    configuration: HashMap<String, String>,
}

impl Config {
    pub fn new() -> Config {
        let config: HashMap<String, String>;
        let json = || -> Result<HashMap<String, String>, Error> {
        // let file = fs::read_to_string("config.json")?;

        let file_name = match dirs::home_dir() {
            Some(x) => format!("{}/gh-releases/config.json", x.to_str().unwrap()),
            None => String::from("config.json"),
        };

        let file = fs::read_to_string(file_name)?;

            let file_contents: Config = serde_json::from_str(&file)?;
            Ok(file_contents.configuration)
        };

        if let Err(_e) = json() {
            config = HashMap::new();
        } else {
            config = json().unwrap();
        }

        Config {
            configuration: config,
        }
    }

    fn save(&self) -> Result<(), Error> {
        let output = serde_json::to_string(self)?;
        let output_bytes = output.as_bytes();
        // let file = fs::File::create("config.json")?;

        let file_name = match dirs::home_dir() {
            Some(x) => format!("{}/gh-releases/config.json", x.to_str().unwrap()),
            None => String::from("config.json"),
        };

        let mut file = fs::File::create(file_name)?;

        match file.write(output_bytes) {
            Ok(_n) => Ok(()),
            _ => Err(format_err!("Problem writing config file")),
        }
    }

    pub fn list(&self) -> Result<Vec<String>, Error> {
        Ok(self
            .configuration
            .iter()
            .map(|(key, value)| format!("{} : {}", key, value))
            .collect())
    }

    pub fn set_release_dir(&mut self, dir: &str) -> Result<(), Error> {
        self.set_or_update("release_dir", dir)
    }

    pub fn get_release_dir(&self) -> String {
        match self.configuration.contains_key("release_dir") {
            true => self.configuration.get("release_dir").unwrap().clone(),
            false => {
                match dirs::home_dir() {
                    Some(x) => return format!("{}/gh-releases", x.to_str().unwrap()),
                    None => return String::from("gh-releases"),
                }
            }
        }
    }

    pub fn unset_release_dir(&mut self) -> Result<(), Error> {
        self.remove("release_dir")
    }

    fn set_or_update(&mut self, key: &str, value: &str) -> Result<(), Error> {
        self.configuration
            .insert(key.to_string(), value.to_string());
        self.save()
    }

    fn remove(&mut self, key: &str) -> Result<(), Error> {
        self.configuration.remove(key);
        self.save()
    }
}
