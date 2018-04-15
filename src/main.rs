extern crate chrono;
extern crate hubcaps;
extern crate config;
extern crate tokio_core;
extern crate futures;

use hubcaps::{Credentials, Github};
use tokio_core::reactor::Core;
use futures::stream::Stream;

pub mod configuration;


fn human_days_ago(days_ago: i64) -> String {
    match days_ago {
        0 => "today".to_string(),
        1 => "yesterday".to_string(),
        _ => format!("{} days ago", days_ago.to_string()),
    }
}

fn main() {
    let config = configuration::Configuration::new();
    let mut core = Core::new().unwrap();
    let github = Github::new("myPrs", Some(Credentials::Token(config.access_token)), &core.handle());

    println!("Checking PRs in your repos");
    for repo_name in config.repos.iter() {
        let vect: Vec<&str> = repo_name.split("/").collect();
        let repo = github.repo(vect[0], vect[1]);
        let pulls = repo.pulls();
        core.run(
            pulls
            .iter(&Default::default())
            .for_each(|pull| {
                let date = chrono::DateTime::parse_from_rfc3339(&pull.updated_at)
                    .unwrap()
                    .date();
                let now = chrono::Utc::now().date();
                let diff = now.signed_duration_since(date);

                println!("On {}", repo_name);
                println!("{}: {}", human_days_ago(diff.num_days()), pull.title);
                println!("             {}", pull.html_url);
                println!("             {}", pull.user.login);
                println!();
                Ok(())
            }),
        ).unwrap();    
    }
}
