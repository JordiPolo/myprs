extern crate config;

use std;

pub struct Configuration {
    pub repos: Vec<String>,
    pub access_token: String
}

impl Configuration {
    pub fn new() -> Configuration {
        config::merge(config::File::new(&filename(), config::FileFormat::Yaml));
        Configuration {repos: repos_list(), access_token: access_token()}
    }
}

// This is private stuff
fn filename() -> String {
    let mut path = std::env::home_dir().unwrap();
    path.push(".merrow.yml");
    path.to_str().unwrap().to_owned()
}

fn access_token() -> String {
    config::get_str("access_token")
    .expect("access token not found.")
    .into_owned()
}

fn repos_list() -> std::vec::Vec<String> {
    config::get_slice("repos").expect("repos not found").iter().map(|a| {
        a.as_str().unwrap().into_owned()
    }).collect()
}
