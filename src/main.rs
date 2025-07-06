mod rng;
mod util;

use std::env;

use rng::{rand, Random};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut whole = false;
    let mut start = 0u128;
    let mut end = 1u128;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-w" => {
                whole = true;
                i += 1;
            }
            _ => {
                if i + 1 < args.len() {
                    start = args[i].parse().unwrap_or(0);
                    end = args[i + 1].parse().unwrap_or(1);
                    i += 2;
                } else {
                    eprintln!("Expected two numbers after the range");
                    std::process::exit(1);
                }
            }
        }
    }

    let r = Random::new(start, end);

    let mut random_num = rand(&r).unwrap();
    if whole {
        random_num = random_num.round();
    }
    println!("{}", random_num);
}
