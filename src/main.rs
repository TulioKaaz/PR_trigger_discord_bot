use octocrab::{etag::Etagged, models::events::Event, Octocrab, Page, Result};
use std::{thread, time, env};
use dotenv::dotenv;

struct RepoData {
    owner: String,
    repo: String,
}

async fn monitor_pull_requests(repo_data: RepoData, github_token: String) -> Result<Etagged<Page<Event>>> {
    let delay = time::Duration::from_secs(10);
    let octocrab = Octocrab::builder().personal_token(github_token).build()?;
    let mut etag = None;
    loop {
        let response = octocrab.repos(&repo_data.owner, &repo_data.repo).events().etag(etag).send().await?;
        if let Some(page) = response.value {
            for event in page.items {
                println!("{:?}", event);
            }
        } else {
            println!("No New Data Received");
        }
        etag = response.etag;
        thread::sleep(delay);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let repo_data = RepoData {
        owner: "TulioKaaz".to_string(),
        repo: "PR_trigger_discord_bot".to_string(),
    };
    let github_token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN is not set in .env file");


    monitor_pull_requests(repo_data, github_token).await.unwrap();
}
