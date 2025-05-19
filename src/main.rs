use reqwest::blocking::get;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};

// JSON構造体のデシリアライズ用
#[derive(Debug, Deserialize)]
struct GeyserResponse {
    downloads: Downloads,
}

#[derive(Debug, Deserialize)]
struct Downloads {
    #[serde(rename = "spigot")]
    spigot: DownloadDetail,
    #[serde(rename = "fabric")]
    fabric: DownloadDetail,
    #[serde(rename = "bungeecord")]
    bungeecord: DownloadDetail,
    #[serde(rename = "velocity")]
    velocity: DownloadDetail,
}

#[derive(Debug, Deserialize)]
struct DownloadDetail {
    sha256: String,
}

fn get_latest_sha256(download_type: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://download.geysermc.org/v2/projects/geyser/versions/latest/builds/latest";
    let resp = get(url)?.json::<GeyserResponse>()?;
    let sha256 = match download_type {
        "spigot" => &resp.downloads.spigot.sha256,
        "fabric" => &resp.downloads.fabric.sha256,
        "bungeecord" => &resp.downloads.bungeecord.sha256,
        "velocity" => &resp.downloads.velocity.sha256,
        _ => return Err("Invalid type".into()),
    };
    Ok(sha256.clone())
}

fn get_file_sha256(path: &str) -> Option<String> {
    let file = File::open(path).ok()?;
    let mut hasher = Sha256::new();
    let mut reader = std::io::BufReader::new(file);
    let mut buffer = [0; 4096];

    loop {
        let n = reader.read(&mut buffer).ok()?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Some(hex::encode(hasher.finalize()))
}

fn download_latest(download_type: &str, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("https://download.geysermc.org/v2/projects/geyser/versions/latest/builds/latest/downloads/{}", download_type);
    let response = get(&url)?;
    let bytes = response.bytes()?;

    let path_obj = Path::new(path);
    if let Some(parent) = path_obj.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    file.write_all(&bytes)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let download_type = "velocity";
    let path = "./Geyser.jar";

    let sha_cloud = get_latest_sha256(download_type)?;
    let sha_local = get_file_sha256(path);

    if sha_local.as_deref() != Some(&sha_cloud) {
        println!("GeyserMC {} Download...", download_type);
        download_latest(download_type, path)?;
        println!("{} Done", path);
    } else {
        println!("Already up-to-date.");
    }

    Ok(())
}
