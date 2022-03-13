use crate::error::*;
use crate::settings::Settings;
use base64::encode;

struct ClientKey(String);

impl ClientKey {
    fn new(key: String) -> Result<Self> {
        match key.as_str() {
            "" => Err(OmniRssError::MissingConfigValue("spotify key")),
            k if k.len() != 32 => Err(OmniRssError::BadKeyLength),
            k if k.to_ascii_lowercase() != k => Err(OmniRssError::BadKeyCharacters),
            k if k.to_lowercase() != k => Err(OmniRssError::BadKeyCharacters),
            _ => Ok(ClientKey(key)),
        }
    }
}

pub struct SpotifyCredentials {
    client_id: ClientKey,
    client_secret: ClientKey,
}

impl SpotifyCredentials {
    pub fn from_config(config: &Settings) -> Result<Self> {
        let id = ClientKey::new(config.spotify.client_id.clone())?;
        let secret = ClientKey::new(config.spotify.client_secret.clone())?;

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
    use crate::error::OmniRssError;
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
            Err(OmniRssError::MissingConfigValue("spotify key"))
        ));
        assert!(matches!(
            ClientKey::new("1234567890".to_string()),
            Err(OmniRssError::BadKeyLength)
        ));
        assert!(matches!(
            ClientKey::new(SAMPLE_KEY.to_uppercase()),
            Err(OmniRssError::BadKeyCharacters)
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
