extern crate yaml_rust;

use std;
use std::env;
use yaml_rust::yaml;
use std::io::prelude::*;

pub struct Configuration {
    pub repos: Vec<String>,
    pub access_token: String
}

impl Configuration {
    pub fn new() -> Configuration {
        let yaml_data = yaml_as_hash();
        let access_token = access_token(&yaml_data);
        let repos = repos_list(&yaml_data);
        Configuration {repos: repos, access_token: access_token}
    }
}

// This is private stuff

fn filename() -> String {
    let mut path = env::home_dir().unwrap();
    path.push(".merrow.yml");
    path.to_str().unwrap().to_owned()
}


fn yaml_as_hash() -> yaml_rust::yaml::Hash {
    let mut f = std::fs::File::open(filename()).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let docs = yaml::YamlLoader::load_from_str(&s).unwrap();
    let my_doc = &docs[0];
    my_doc.clone().into_hash().unwrap()
}

fn access_token(yaml_data: &yaml_rust::yaml::Hash) -> String {
    let error_msg = "The configuration file needs a key access_token with the access token to Github";
    let string_access = yaml::Yaml::String("access_token".to_string());
    let access_token_str = yaml_data.get(&string_access).expect(error_msg);
    access_token_str.as_str().expect(error_msg).to_string()
}

fn repos_list(yaml_data: &yaml_rust::yaml::Hash) -> std::vec::Vec<String> {
    let string = yaml::Yaml::String(":repos".to_string());
    let data_array = yaml_data.get(&string).unwrap();
    let data = data_array.as_vec().unwrap().clone();
    data.into_iter().map(|a| a.as_str().unwrap().to_string()).collect()
   // data
}
