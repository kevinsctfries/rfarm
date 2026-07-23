mod crop;
mod farm;

use farm::Farm;
use std::{
    thread,
    time::Duration,
};


fn main() {

    let mut farm = Farm::new();

    loop {

        farm.tick();

        println!(
            "Age: {} | Height: {}",
            farm.crop.age,
            farm.crop.height
        );

        thread::sleep(
            Duration::from_secs(1)
        );

    }

}