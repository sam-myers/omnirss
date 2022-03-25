use crate::spotify::response::GetShow;
use std::fs::File;
use std::path::Path;

impl GetShow {
    pub fn test_fixture_joe_rogan() -> Box<Self> {
        let path = Path::new("fixtures/spotify/joe_rogan_show.json");
        let file = File::open(&path).expect("fixture missing");
        serde_json::from_reader(file).expect("parse failed")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fixture_joe_rogan_exists() {
        crate::spotify::response::GetShow::test_fixture_joe_rogan();
    }
}
