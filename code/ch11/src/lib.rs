pub fn string_from() {
    let mut king = String::from("King ");
    king.push_str("Crab");
    king.push('!');
    println!("{king}");
}
pub fn option_demo() {
    let b1: Option<bool> = None;
    let b2: Option<bool> = Some(true);
    println!("b1 = {b1:?}; b2 = {b2:?}");
    assert_ne!(b1, b2);
}

pub fn result_demo() {
    let r1: Result<u8, &str> = Ok(42);
    let r2: Result<u8, &str> = Err("Help!");
    assert_ne!(r1, r2);
}
