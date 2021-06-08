use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use rss::Channel;

async fn get_rss(url: &str) -> Result<Channel, Box<dyn std::error::Error>> {
    let content = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;
    Ok(channel)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("configs/urls.txt");
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        let url = line?.to_string();

        let channel = get_rss(&url).await?;

        for item in channel.into_items() {
            println!(
                "\nTitle: {}\nDescription: {}\n",
                item.title().unwrap(),
                item.description().unwrap()
            );
            println!();
        }
    }

    Ok(())
}
