use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaFilters {
    pub media_type: String,
    pub page_size: i64,
    pub cursor_id: Option<i64>,
    pub only_watched: Option<bool>,
    pub include_watched: Option<bool>,
    pub only_watchlisted: Option<bool>,
    pub only_favourites: Option<bool>,
    pub only_hidden: Option<bool>,
    pub include_hidden: Option<bool>,
    pub include_favourites: Option<bool>,
    pub include_watchlisted: Option<bool>,
    pub min_year: Option<i32>,
    pub max_year: Option<i32>,
    pub min_avg_rating: Option<f64>,
    pub max_avg_rating: Option<f64>,
    pub min_letterboxd_rating: Option<f64>,
    pub max_letterboxd_rating: Option<f64>,
    pub min_metacritic: Option<f64>,
    pub max_metacritic: Option<f64>,
    pub min_imdb_rating: Option<f64>,
    pub max_imdb_rating: Option<f64>,
    pub min_tomatometer: Option<f64>,
    pub max_tomatometer: Option<f64>,
    pub min_popcornmeter: Option<f64>,
    pub max_popcornmeter: Option<f64>,
    pub min_count: Option<f64>,
    pub max_count: Option<f64>,
    pub genres: Option<Vec<String>>,
    pub exclude_genres: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
    pub exclude_languages: Option<Vec<String>>,
    pub countries: Option<Vec<String>>,
    pub exclude_countries: Option<Vec<String>>,
    pub status: Option<Vec<String>>,
    pub content_ratings: Option<Vec<String>>,
    pub genders: Option<Vec<String>>,
    pub postgres_id: Option<String>,
    pub sort: Option<String>,
    pub ascending: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpsertUserMediaData {
    pub postgres_id: String,
    pub media_id: i32,
    #[serde(default)]
    pub watched: Option<bool>,
    #[serde(default)]
    pub hidden: Option<bool>,
    #[serde(default)]
    pub in_collection: Option<bool>,
    #[serde(default)]
    pub in_watchlist: Option<bool>,
    #[serde(default)]
    pub episode_id: Option<i32>,
    #[serde(default)]
    pub progress: Option<i32>,
    #[serde(default)]
    pub rating: Option<i16>,
    #[serde(default)]
    pub episode_group_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiAuthResult {
    pub postgres_id: String,
    pub email: String,
    pub username: Option<String>,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: ApiAuthResult,
    pub token: String,
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    pub search_term: String,
    pub page: Option<i64>,
    pub page_size: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FetchMediaRequest {
    pub external_ids: ExternalIds,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalIds {
    pub imdb: ImdbId,
    pub tmdb: TmdbId,
    pub tvdb: TvdbId,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImdbId {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TmdbId {
    pub id: Option<i32>,
    pub r#type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TvdbId {
    pub id: Option<i32>,
    pub r#type: Option<String>,
}
