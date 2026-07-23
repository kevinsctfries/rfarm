mod world;

use std::env;
use std::thread;
use std::time::Duration;

use std::io::{self, Write};
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

    let mut map = WorldGenerator::generate(80, 40, seed);

    loop {
        print!("\x1b[?25h");
        io::stdout().flush().unwrap();

        Renderer::render(&map);

        map.update_vehicles();

        thread::sleep(Duration::from_millis(500));
    }
}
