pub mod rng;
pub mod util;

pub use rng::{MersenneTwister64, Random, rand};
pub use util::{wrap_number, get_time};
