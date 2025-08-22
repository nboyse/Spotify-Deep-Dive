use actix_files as fs;
use actix_web::{get, web, App, HttpServer, Responder};
use serde::Serialize;
use serde::Deserialize;
use dotenvy::dotenv;
use std::env;

struct AppState {
    spotify_token: String,
    app_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Artist {
    id: String,
    name: String,
    genres: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Track {
    name: String,
}

#[derive(Deserialize, Debug)]
struct TopTracksResponse {
    tracks: Vec<Track>,
}

#[derive(Deserialize, Debug)]
struct TopArtistsResponse {
    items: Vec<Artist>,
}

#[derive(Serialize)]
struct GenreCount {
    genre: String,
    count: usize,
}

#[derive(Serialize)]
struct GenreDetail {
    artist: String,
    tracks: Vec<String>,
}

#[get("/genres")]
async fn genres(data: web::Data<AppState>) -> impl Responder {
    let client = reqwest::Client::new();
    let url = "https://api.spotify.com/v1/me/top/artists?limit=20";
    
    let resp = client
    .get(url)
    .bearer_auth(&data.spotify_token)
    .send()
    .await
    .unwrap();

    let text = resp.text().await.unwrap();
    println!("{}", text);

    let resp: TopArtistsResponse = serde_json::from_str(&text).unwrap();

    println!("{:?}", resp);
    // flatten all genres
    let mut counts = std::collections::HashMap::new();

    for artist in resp.items {
        for genre in artist.genres {
            *counts.entry(genre).or_insert(0) += 1;
        }
    }

    // turn vector for json
    let mut result: Vec<_> = counts
        .into_iter()
        .map(|(genre, count)| GenreCount { genre, count })
        .collect();

    // makes it sorted descending
    result.sort_by_key(|g| std::cmp::Reverse(g.count));
    
    web::Json(result)
}

#[get("/genres/{genre}")]
async fn genre_detail(
    data: web::Data<AppState>,
    path: web::Path<String>
) -> impl Responder {
    let genre = path.into_inner();
    let client = reqwest::Client::new();
    let url = "https://api.spotify.com/v1/me/top/artists?limit=20";

    let resp = client
        .get(url)
        .bearer_auth(&data.spotify_token)
        .send()
        .await
        .unwrap()
        .json::<TopArtistsResponse>()
        .await
        .unwrap();

    let mut details = Vec::new();

    for artist in resp.items {
        if artist.genres.contains(&genre) {
            let track_url = format!("https://api.spotify.com/v1/artists/{}/top-tracks?market=US", artist.id);

            let track_resp = client
                .get(&track_url)
                .bearer_auth(&data.spotify_token)
                .send()
                .await
                .unwrap()
                .json::<TopTracksResponse>()
                .await
                .unwrap();

            let track_names = track_resp.tracks.into_iter().map(|t| t.name).collect();

            details.push(GenreDetail {
                artist: artist.name,
                tracks: track_names,
            });
        }
    }

    web::Json(details)
}

#[get("/viz")]
async fn viz() -> impl Responder {
    let html = r#"
    
    "#;
    actix_web::HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let token = env::var("SPOTIFY_TOKEN").expect("SPOTIFY_TOKEN must be set in .env file");
    
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                spotify_token: token.clone(),
                app_name: String::from("Actix Web"),
            }))
            .service(index)
            .service(genres)
            .service(genre_detail)
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .route("/viz", web::get().to(|| async { fs::NamedFile::open("./static/index.html")}))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}