use rand::RngExt;

use std::fs;
use std::path::Path;

const SEED_FILE: &str = "world.seed";

#[derive(Clone, Copy)]
pub struct Seed(pub u64);

impl Seed {
    pub fn load_or_generate() -> Seed {
        if Path::new(SEED_FILE).exists() {
            let value = fs::read_to_string(SEED_FILE)
                .expect("Failed to read seed")
                .trim()
                .parse::<u64>()
                .expect("Invalid seed file");

            return Seed(value);
        }

        Self::generate()
    }

    pub fn generate() -> Seed {
        let seed = rand::rng().random::<u64>();

        fs::write(SEED_FILE, seed.to_string()).expect("Failed to save seed");

        println!("Generated new world seed: {}", seed);

        Seed(seed)
    }

    pub fn generate_new() -> Seed {
        let seed = rand::rng().random::<u64>();

        fs::write(SEED_FILE, seed.to_string()).expect("Failed to save new seed");

        println!("Generated new world seed: {}", seed);

        Seed(seed)
    }
}
