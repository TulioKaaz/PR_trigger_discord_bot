use octocrab::{Octocrab, Result};

struct RepoData {
    owner: String,
    repo: String,
}

async fn monitor_pull_requests(repo_data: RepoData, github_token: str) -> Result<()> {
    let octocrab = Octocrab::builder().personal_token(github_token).build()?;

    let mut stream = octocrab.repos(owner, repo).events().per_page(100).send().await?;

    stream
}

fn main() {
    monitor_pull_requests(RepoData {
        owner: "TulioKaaz",
        repo: "octocrab",
    }, "ghp_...").await;
    println!("{}", );
}
