
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::thread::sleep;
use std::time::Duration;

use crate::models::Movie;

pub fn build_client() -> Client {
    Client::builder()
        .user_agent(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
             AppleWebKit/537.36 (KHTML, like Gecko) \
             Chrome/124.0.0.0 Safari/537.36",
        )
        .timeout(Duration::from_secs(30))
        .build()
        .expect("Failed to build HTTP client")
}

fn fetch_page(client: &Client, url: &str) -> Result<String, Box<dyn std::error::Error>> {
    let response = client.get(url).send()?;
    if !response.status().is_success() {
        return Err(format!("HTTP {} for {}", response.status(), url).into());
    }

    Ok(response.text()?)
}


fn parse_watchlist_page(html: &str) -> Vec<Movie> {
    let document = Html::parse_document(html);

    let li_selector = Selector::parse("ul.grid li.griditem").unwrap();
    let component_selector = Selector::parse("div.react-component").unwrap();

    let mut movies = Vec::new();

    for li in document.select(&li_selector) {
        let component = match li.select(&component_selector).next() {
            Some(el) => el,
            None => continue,
        };

        let film_id = match component.value().attr("data-film-id") {
            Some(id) => id.to_string(),
            None => continue,
        };

        let raw_name = match component.value().attr("data-item-name") {
            Some(name) => decode_html_entities(name),
            None => continue,
        };

        let slug = match component.value().attr("data-item-slug") {
            Some(s) => s.to_string(),
            None => continue,
        };

        let id_path = film_id
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("/");

        let poster_url = Some(format!(
                "https://a.ltrbxd.com/resized/film-poster/{}/{}-{}-0-125-0-187-crop.jpg",
                id_path, film_id, slug
        ));


        movies.push(Movie::new(film_id, raw_name, slug, poster_url));
    }
    movies
}

fn decode_html_entities(s: &str) -> String {
    s.replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("quot;", "\"")
        .replace("&#39;", "'")
}

pub fn scrap_watchlist(client: &Client, username: &str) -> Result<Vec<Movie>, Box<dyn std::error::Error>> {
    let mut all_movies: Vec<Movie> = Vec::new();
    let mut page = 1;

    loop {
        let url = format!(
            "https://letterboxd.com/{}/watchlist/page/{}/",
            username, page
        );

        println!("Fetching page {}...", page);

        let html = fetch_page(client, &url)?;
        let movies_on_page = parse_watchlist_page(&html);

        if movies_on_page.is_empty() {
            println!("No more movies found, done.");
            break;
        }

        println!("Found {} movies on this page.", movies_on_page.len());
        all_movies.extend(movies_on_page);
        page += 1;

        sleep(Duration::from_millis(800));
    }

    Ok(all_movies)
}





