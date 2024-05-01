// #[path = "lib.rs"]
// mod sea::mod;

use ch06_simple_path::{pub_use_demo, sea};

fn main() {
    //   sea::world::crab();
    sea::world::crab();
    pub_use_demo();
}
