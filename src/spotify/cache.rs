pub struct CacheKey {}

pub const CACHE_SHOW_FOR_SECONDS: usize = 1200;

impl CacheKey {
    pub fn show_from_id(show_id: &String) -> String {
        format!("spotify:id:{}", show_id)
    }
}
