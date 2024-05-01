#[path ="my_mod_b.rs"]
mod my_mod_b;

pub fn print_b() {
  println!("{}", my_mod_b::B);
}