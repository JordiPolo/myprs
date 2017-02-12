extern crate hyper;
extern crate hubcaps;
extern crate chrono;

use hyper::Client;
use hubcaps::{Credentials, Github};

pub mod configuration;


fn pull_requests(github: &hubcaps::Github, repo_name: &str) -> std::vec::Vec<hubcaps::Pull> {
    let vect: Vec<&str> = repo_name.split("/").collect();
    let repo = github.repo(vect[0], vect[1]);
    repo.pulls().list(&Default::default()).unwrap()
}

fn human_days_ago(days_ago: i64) -> String {
    match(days_ago) {
        0 => "today".to_string(),
        1 => "yesterday".to_string(),
        _ => format!("{} days ago", days_ago.to_string())
    }
}

fn main() {
    let config = configuration::Configuration::new();
    let client = Client::new();
    let github = Github::new("myPrs", &client, Credentials::Token(config.access_token));

    println!("Checking PRs in your repos");
    for repo_name in config.repos.iter() {
        //println!("{:?}",repo_name);
        for pull in pull_requests(&github, repo_name) {
             let date = chrono::DateTime::parse_from_rfc3339(&pull.updated_at).unwrap().date();
             let now = chrono::UTC::now().date();
             let diff = now.signed_duration_since(date);

             println!("On {}", repo_name);
             println!("{}: {}", human_days_ago(diff.num_days()), pull.title);
             println!("             {}", pull.html_url);
             println!("             {}", pull.user.login);
             println!();
         }
    }


}

