pub fn lifetime_demo_1<'a>() -> &'a str {
    let x = "hello";
    let y: &'a str = &x;
    y
}

pub fn rust_shadowing_demo() {
  let x = 21;
  println!("{x}");
  let x = 42;
  println!("{x}");
  }