pub struct CacheKey {}

pub const CACHE_SHOW_FOR_SECONDS: usize = 120;
pub const CACHE_SEARCH_FOR_SECONDS: usize = 600;

impl CacheKey {
    pub fn show(show_id: &str) -> String {
        format!("spotify:id:{}", show_id.to_lowercase())
    }

    pub fn search(query: &str) -> String {
        format!("spotify:search:{}", query.to_lowercase())
    }
}
