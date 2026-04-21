mod models;
mod storage;
mod scraper;

use clap::{Parser, Subcommand};
use rand::seq::SliceRandom;

#[derive(Parser)]
#[command(name = "Letterboxd Tool")]
#[command(about = "Un outil pour gérer et scraper les watchlists letterboxd", long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Scrap {
        username: String
    },
    Count,
    Info {
        name: String,
    },
    Random,
    Recommend {
        prompt: String,
    },
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let file_path = "watchlist.json";
    
    match cli.command {
        Command::Scrap { username } => {
            let username = username.to_lowercase();

            // Charger la watchlist
            let mut watchlist = storage::load_watchlist(file_path)?;

            // Scrap
            println!("Starting craping of {}", username);
            println!("🤖 Réflexion de l'IA en cours (cela peut prendre jusqu'à 1 minute)...");
            
            let client = reqwest::blocking::Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()?;
            let scraped_movies = scraper::scrap_watchlist(&client, &username)?;

            // Ajouter les films si ils ne sont pas déjà dans le json
            let mut new_additions = 0;
            for movie in scraped_movies {
                if watchlist.add_if_new(movie) {
                    new_additions += 1
                }
            }

            // Sauvegarder
            if new_additions > 0 {
                println!("Added {} new movies", new_additions);
                storage::save_watchlist(file_path, &watchlist)?;
            } else {
                println!("No new movies to add.");
            }
        }

        Command::Count => {
            let watchlist = storage::load_watchlist(file_path)?;
            println!("{} movies loaded", watchlist.movies.len());
        } 

        Command::Info { name } => {
            let watchlist = storage::load_watchlist(file_path)?;
            let search_name = name.to_lowercase().replace(" ", "");

            let found_movie = watchlist.movies.iter().find(|m| {
                let movie_title = m.title.to_lowercase().replace(" ", "");
                movie_title.contains(&search_name)
            });

            match found_movie {
                Some(m) => {
                    println!("--- Info Movie ---");
                    println!("Title : {} ({})", m.title, m.year.unwrap_or(0));
                    println!("ID : {}", m.film_id);
                    println!("link : https://letterboxd.com/film/{}/", m.slug);
                    println!("Poster : {}", m.poster_url.as_deref().unwrap_or("Pas d'image"));
                }
                None => println!("No movie find with this name '{}'", name),
            }
        }

        Command::Random => {
            let watchlist = storage::load_watchlist(file_path)?;

            if watchlist.movies.is_empty() {
                println!("Watchlist is empty.");
                return Ok(());
            }

            let mut rng = rand::thread_rng();

            if let Some(m) = watchlist.movies.choose(&mut rng) {
                println!("--- Random Movie ---");
                println!("Title : {} ({})", m.title, m.year.unwrap_or(0));
                println!("ID : {}", m.film_id);
                println!("link : https://letterboxd.com/film/{}/", m.slug);
                println!("Poster : {}", m.poster_url.as_deref().unwrap_or("No image"));
            }
        }

        Command::Recommend { prompt } => {
            let mut child = std::process::Command::new("python3")
                .arg("recommander.py")
                .arg(&prompt)
                .spawn()?;

            child.wait()?;
        }
    }
        
    Ok(())
}
