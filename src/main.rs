mod world;

use std::env;

use world::generator::WorldGenerator;
use world::renderer::Renderer;
use world::seed::Seed;

fn main() {
    let args: Vec<String> = env::args().collect();

    let seed = if args.len() > 1 && args[1] == "new" {
        Seed::generate_new()
    } else {
        Seed::load_or_generate()
    };

    println!("Using world seed: {}", seed.0);

    let map = WorldGenerator::generate(80, 40, seed);

    Renderer::render(&map);
}
