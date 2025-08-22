use serde::Serialize;
use serde::Deserialize;

pub struct AppState {
    pub spotify_token: String,
    pub app_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Artist {
    pub id: String,
    pub name: String,
    pub genres: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct Track {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct TopTracksResponse {
    pub tracks: Vec<Track>,
}

#[derive(Deserialize, Debug)]
pub struct TopArtistsResponse {
    #[serde(default)]
    pub items: Vec<Artist>,
}

#[derive(Serialize)]
pub struct GenreCount {
    pub genre: String,
    pub count: usize,
}

#[derive(Serialize)]
pub struct GenreDetail {
    pub artist: String,
    pub tracks: Vec<String>,
}

#[derive(Serialize)]
pub struct Recommendation {
    pub artist: String,
    pub track: String,
    pub spotify_url: String,
}