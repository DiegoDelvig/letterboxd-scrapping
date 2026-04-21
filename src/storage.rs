
use std::fs;
use std::path::Path;
use crate::models::Watchlist;

pub fn load_watchlist(path: &str) -> Result<Watchlist, Box<dyn std::error::Error>> {
    if !Path::new(path).exists() {
        println!("No existing watchlist, starting fresh !");
        return Ok(Watchlist::default());
    }

    println!("Loading existing watchlist...");
    let content = fs::read_to_string(path)?;
    let watchlist:  Watchlist = serde_json::from_str(&content)?;
    println!("Found {} movies", watchlist.movies.len());
    Ok(watchlist)
}


pub fn save_watchlist(path: &str, watchlist: &Watchlist) -> Result<(), Box<dyn std::error::Error>> {
    println!("Saving watchlist to {}", path);

    let content = serde_json::to_string_pretty(watchlist)?;
    fs::write(path, content)?;
    println!("Watchlist saved successfully!");
    Ok(())
}
