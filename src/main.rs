mod rng;
mod util;

use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

use rng::{rand, Random};

fn main() {
    println!("Hello, world!");

    let mut map = HashMap::new();
    for _ in 0..100 {
        let random_num = rand(Random::new(0, 100)).expect("rand failed").round() as u128;
        println!("{}", random_num);
        *map.entry(random_num).or_insert(0) += 1;
    }

    loop {
        let random_num = rand(Random::default()).unwrap();  
        sleep(Duration::from_micros(100_000));
        println!("{}", random_num);
    } 
}
