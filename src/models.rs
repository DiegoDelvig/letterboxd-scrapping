use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Movie {
    pub film_id: String,
    pub title: String,
    pub year: Option<u32>,
    pub slug: String,
    pub poster_url: Option<String>,
    pub genres: Vec<String>,
}

impl Movie {
    pub fn new(
        film_id: String,
        raw_name: String,
        slug: String,
        poster_url: Option<String>
    ) -> Self {
        let (title, year) = parse_title_year(&raw_name);
        Movie {
            film_id,
            title,
            year,
            slug,
            poster_url,
            genres: Vec::new(),
        }
    }
}

fn parse_title_year(raw: &str) -> (String, Option<u32>) {
    if let Some(open) = raw.rfind(" (") {
        let rest = &raw[open + 2..];
        if let Some(close) = rest.find(")") {
            if let Ok(year) = rest[..close].parse::<u32>() {
                return (raw[..open].to_string(), Some(year))
            }
        }
    }
    (raw.to_string(), None)
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Watchlist {
    pub movies: Vec<Movie>,
}

impl Watchlist {
    pub fn contains(&self, film_id: &str) -> bool {
       self.movies.iter().any(|m| m.film_id == film_id) 
    }

    pub fn add_if_new(&mut self, movie: Movie) -> bool {
        if self.contains(&movie.film_id) {
            false
        } else {
            self.movies.push(movie);
            true
        }
    }
}

