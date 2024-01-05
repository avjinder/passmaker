use std::env;

use clap::Parser;
use passmaker::{config::PassConfig, generator::PassGenerator};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let config = PassConfig::parse();
    let generator = PassGenerator::new(&config);

    for pass in generator.take(config.count) {
        println!("{pass}");
    }
}
