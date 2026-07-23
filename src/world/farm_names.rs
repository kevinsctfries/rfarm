use rand::RngExt;
use serde::Deserialize;

#[derive(Deserialize)]
struct FarmNameData {
    prefixes: Vec<String>,
    suffixes: Vec<String>,
}

pub struct FarmNameGenerator {
    data: FarmNameData,
}

impl FarmNameGenerator {
    pub fn load() -> Self {
        let json = include_str!("../data/farm_names.json");

        let data: FarmNameData = serde_json::from_str(json).expect("Failed to load farm names");

        Self { data }
    }

    pub fn generate(&self, rng: &mut impl RngExt) -> String {
        let prefix = &self.data.prefixes[rng.random_range(0..self.data.prefixes.len())];

        let suffix = &self.data.suffixes[rng.random_range(0..self.data.suffixes.len())];

        format!("{} {}", prefix, suffix)
    }
}
