use anyhow::Result;
use reqwest;
use tokio;
use proton_upd::parse_downloads;
use clap::Parser;

/// A simple CLI tool to download the latest version of Proton Drive CLI for a specified platform.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long,)] // default_value_t = String::from("linux/x64"
    platform: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let res = reqwest::get("https://proton.me/download/drive/cli/index.html").await?;
    println!("Status: {}", res.status());
    //println!("Headers:\n{:#?}", res.headers());

    let body = res.text().await?;
    let downloads = parse_downloads(&body)
        .ok_or(anyhow::anyhow!("Failed to parse downloads"))?;
    // println!("Downloads: {:#?}", downloads);
    /* for download in downloads {
        println!("Platform: {}", download.platform);
        println!("URL: {}", download.url);
        println!("Checksum: {}", download.checksum);
        println!();
    } */

    
    Ok(())
}
