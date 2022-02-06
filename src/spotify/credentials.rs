use crate::error::*;
use base64::encode;

struct ClientKey(String);

impl ClientKey {
    fn new(key: String) -> Result<Self> {
        match key.as_str() {
            "" => Err(SpotiRssError::MissingKey),
            k if k.len() != 32 => Err(SpotiRssError::BadKeyLength),
            k if k.to_ascii_lowercase() != k => Err(SpotiRssError::BadKeyCharacters),
            k if k.to_lowercase() != k => Err(SpotiRssError::BadKeyCharacters),
            _ => Ok(ClientKey(key)),
        }
    }
}

pub struct SpotifyCredentials {
    client_id: ClientKey,
    client_secret: ClientKey,
}

impl SpotifyCredentials {
    pub fn from_env_vars() -> Result<Self> {
        let id = ClientKey::new(std::env::var("SPOTIFY_CLIENT_ID")?)?;
        let secret = ClientKey::new(std::env::var("SPOTIFY_CLIENT_SECRET")?)?;

        Ok(Self {
            client_id: id,
            client_secret: secret,
        })
    }

    pub fn basic_auth_header(&self) -> String {
        format!(
            "Basic {}",
            encode(format!("{}:{}", self.client_id.0, self.client_secret.0))
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::error::SpotiRssError;
    use crate::spotify::credentials::{ClientKey, SpotifyCredentials};

    const SAMPLE_KEY: &str = "abc123abc4567abc890ab12345678901";

    #[test]
    fn test_valid_key() {
        assert!(matches!(ClientKey::new(SAMPLE_KEY.to_string()), Ok(_)));
    }

    #[test]
    fn test_invalid_key() {
        assert!(matches!(
            ClientKey::new("".to_string()),
            Err(SpotiRssError::MissingKey)
        ));
        assert!(matches!(
            ClientKey::new("1234567890".to_string()),
            Err(SpotiRssError::BadKeyLength)
        ));
        assert!(matches!(
            ClientKey::new(SAMPLE_KEY.to_uppercase()),
            Err(SpotiRssError::BadKeyCharacters)
        ));
    }

    #[test]
    fn test_basic_auth_header() {
        let client_id = ClientKey::new(SAMPLE_KEY.to_string()).unwrap();
        let client_secret = ClientKey::new(SAMPLE_KEY.to_string()).unwrap();
        let creds = SpotifyCredentials {
            client_id,
            client_secret,
        };

        assert_eq!(creds.basic_auth_header(), "Basic YWJjMTIzYWJjNDU2N2FiYzg5MGFiMTIzNDU2Nzg5MDE6YWJjMTIzYWJjNDU2N2FiYzg5MGFiMTIzNDU2Nzg5MDE=");
    }
}
