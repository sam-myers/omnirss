use chrono::{DateTime, Utc};

pub struct SpotifyToken {
    pub token: String,
    pub expiry: DateTime<Utc>,
}

impl SpotifyToken {
    pub fn bearer_auth_header(&self) -> String {
        format!("Bearer {}", self.token)
    }
}
