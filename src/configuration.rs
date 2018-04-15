use std;
use config::{Config, File, FileFormat};

pub struct Configuration {
    pub repos: Vec<String>,
    pub access_token: String,
}

impl Configuration {
    pub fn new() -> Configuration {
        let mut settings = Config::new();
        settings.merge(File::new(&filename(), FileFormat::Yaml)).unwrap();
        Configuration {
            repos: repos_list(&settings),
            access_token: access_token(&settings),
        }
    }
}

// This is private stuff
fn filename() -> String {
    let mut path = std::env::home_dir().unwrap();
    path.push(".merrow.yml");
    path.to_str().unwrap().to_owned()
}

fn access_token(settings: &Config) -> String {
    settings
        .get::<String>("access_token")
        .expect("access token not found.")
}

fn repos_list(settings: &Config) -> Vec<String> {
    settings.get::<Vec<String>>("repos")
        .expect("repos not found")
        .iter()
        .map(|a| a.as_str().to_string())
        .collect()
}
