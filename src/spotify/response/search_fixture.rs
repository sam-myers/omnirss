use crate::spotify::response::Search;
use std::fs::File;
use std::path::Path;

impl Search {
    pub fn test_fixture_search_joe() -> Box<Self> {
        let path = Path::new("fixtures/spotify/search_joe.json");
        let file = File::open(&path).expect("fixture missing");
        serde_json::from_reader(file).expect("parse failed")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn fixture_joe_rogan_exists() {
        crate::spotify::response::Search::test_fixture_search_joe();
    }
}
