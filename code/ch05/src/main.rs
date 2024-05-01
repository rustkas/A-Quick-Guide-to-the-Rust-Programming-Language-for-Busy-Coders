// #![no_main]

#![crate_name = "ch05"]

mod world;

pub const A: &str = "A";

mod my_mod {
    pub const A: &str = "I'm an A";
}

// #[test]
fn main() {
    println!("{}", my_mod::A);
    world::print_b();
}
