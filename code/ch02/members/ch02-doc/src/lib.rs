#![crate_type = "lib"]

// nested comment example
/*
I'm a comment.
/*
I'm a comment too.
*/
I'm a comment three.
*/

//! I'm about this module.
//! I'm about this module too.

/// I'm a mathematical pi. ①
/// I'm very accurate.
pub const PI: f32 = 3.14;
/**
* I'm an edible pie.
* I'm very edible.
*/
pub const PIE: &str = "American Pie";

#[test]
pub fn demo() {
    /*!
    I'm about the demo function.
    */
    /// I'm about X. ③
    const _X: i32 = 0;
}
#[test]
pub fn demo2() {
    let r#let = 333;
    let doubled = r#let * 2;
    println!("{let} * 2 = {doubled}");
}

pub fn fst<'a>(x: &'a str, _y: &'a str) -> &'a str {
    x
}
#[test]
pub fn integers() {
    use std::any::{Any, TypeId};
    let a: u64 = 0;
    assert_eq!(TypeId::of::<u64>(), a.type_id());
    let b = 0x42;
    assert_eq!(TypeId::of::<i32>(), b.type_id());
    let c = 0o7000i16;
    assert_eq!(TypeId::of::<i16>(), c.type_id());
    let d = 0b1010_0000_u8;
    assert_eq!(TypeId::of::<u8>(), d.type_id());
}

#[test]
pub fn label_demo() {
    'label: for i in 0..10 {
        if i > 5 {
            break 'label;
        }
    }
}

#[test]
// 2.9.4. Tuple index
fn tuple_indices() {
    let crabby = ("crab", "lobster");
    let crab = crabby.0;
    let lobster = crabby.1;

    assert_eq!("crab", crab);
    assert_eq!("lobster", lobster);
    println!("Crab is {crab}.");
    println!("Lobster is {lobster}.");
}

#[test]
// 2.9.5. Floating-point literals
fn floats() {
    use std::any::{Any, TypeId};
    let a: f32 = 10.;
    let b = 100.;
    println!("a + b = {}", a + b);
    assert_eq!(TypeId::of::<f32>(), b.type_id());
    let c = 3.14159265358979;
    assert_eq!(TypeId::of::<f64>(), c.type_id());
    let d = 3E+15f32;
    assert_eq!(TypeId::of::<f32>(), d.type_id());
    let e = 1_0__0___0____f32;
    assert_eq!(TypeId::of::<f32>(), e.type_id());
}

pub fn repeat(_s: &str, _times: i32) -> String {
    todo!();
}

#[test]
// 2.9.7. String literals
fn strings() {
    let emily = "I heard a fly buzz -
When I died.";
    println!("{emily}");
    let william1 = "My heart leaps up";
    let william2 = "My heart \
leaps up";
    println!("{william1} (vs) {william2}");
}

#[test]
// 2.9.9. Byte literals
fn byte_literals() {
    let b1 = b'A';
    let b2 = 65;
    assert_eq!(b1, b2);
}

#[test]
// 2.9.10. Byte string literals
fn byte_strings() {
    let s1 = b"Hello' \"World\"!";
    let s2 = &[
        b'H', b'e', b'l', b'l', b'o', b'\'', b' ', b'"', b'W', b'o', b'r', b'l', b'd', b'"', b'!',
    ];
    assert_eq!(*s1, *s2);
}

#[test]
// 2.9.11. Raw byte string literals
fn raw_byte_strings() {
    let crab1: &'static [u8;40] = br"You cannot teach a crab
to walk straight";
    let crab2 = br##"~`!@#$%^&*(){}[]<>\|\
;:'"#,.\n\t\'\"\\?/\"##;
    let len1 = crab1.len();
    let len2 = crab2.len();
    println!("Crab1 ({len1}): {crab1:?}");
    println!("Crab2 ({len2}): {crab2:?}");
}
