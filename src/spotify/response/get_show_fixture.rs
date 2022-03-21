use crate::spotify::response::GetShow;
use std::fs::File;
use std::path::Path;

impl GetShow {
    pub fn test_fixture_joe_rogan() -> Self {
        let path = Path::new("fixtures/joe_rogan_show.json");
        let file = File::open(&path).expect("fixture missing");
        serde_json::from_reader(file).expect("parse failed")
    }
}
