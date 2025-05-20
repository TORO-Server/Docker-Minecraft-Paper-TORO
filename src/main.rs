use reqwest::blocking::Client;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize, Debug)]
struct Release {
    assets: Vec<Asset>,
}

#[derive(Deserialize, Debug)]
struct Asset {
    browser_download_url: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let repo_url_str = "https://github.com/TORO-Server/InfinityDispense"; // Replace with the actual repository URL

    let parsed_url = Url::parse(repo_url_str)?;

    let segments: Vec<&str> = parsed_url
        .path_segments()
        .map(|c| c.collect())
        .ok_or("Invalid URL path")?;

    if segments.len() < 2 {
        eprintln!("Invalid GitHub repository URL format.");
        return Ok(());
    }

    let owner = segments[0];
    let repo = segments[1];

    let client = Client::new();
    let api_url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        owner, repo
    );

    println!("Fetching latest release for {}/{}", owner, repo);

    let response = client
        .get(&api_url)
        .header("Accept", "application/vnd.github+json")
        .header("User-Agent", "Mozilla/5.0")
        // .header("Authorization", format!("Bearer {}", token))
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()?;

    if response.status().is_success() {
        let release: Release = response.json()?;
        println!("{}", release.assets[0].browser_download_url);
    } else {
        eprintln!("Failed to fetch latest release: {:?}", response);
    }

    Ok(())
}
