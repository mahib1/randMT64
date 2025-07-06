mod rng;
mod util;

use std::{collections::HashMap, thread::sleep, time::Duration, env};

use rng::{rand, Random};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut whole = false;
    let mut start = 0u128;
    let mut end = 1_000_000_000u128;

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
                    end = args[i + 1].parse().unwrap_or(1_000_000_000);
                    i += 2;
                } else {
                    eprintln!("Expected two numbers after the range");
                    std::process::exit(1);
                }
            }
        }
    }

    let r = Random::new(start, end);

    let mut map = HashMap::new();
    for _ in 0..100 {
        let mut random_num = rand(r).expect("rand failed");
        if whole {
            random_num = random_num.round();
        }
        println!("{}", random_num);
        *map.entry(random_num as u128).or_insert(0) += 1;
    }

    loop {
        let mut random_num = rand(r).unwrap();  
        if whole {
            random_num = random_num.round();
        }
        println!("{}", random_num);
        sleep(Duration::from_micros(100_000));
    } 
}
