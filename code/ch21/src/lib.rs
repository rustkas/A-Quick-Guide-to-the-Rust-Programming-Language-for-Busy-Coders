#![allow(dead_code)]

mod demo {
    #[derive(Debug, Clone, Copy)]
    pub struct Drink {
        size: u8, // In ounces or in liters? ①
        kind: &'static str,
    }
    impl Drink {
        pub fn new(size: u8, kind: &'static str) -> Drink {
            Drink { size, kind }
        }
    }
}
pub fn new_drink_demo() {
    let d = demo::Drink::new(16, "diet water");
    println!("{d:?}");
}

#[derive(Debug)]
struct ComboMeal<Meat, const N: usize> {
    drink: demo::Drink,
    burgers: [Meat; N],
}
pub fn order_super_meal() {
    let meal = ComboMeal {
        burgers: ["turkey", "goat", "tuna"],
        drink: demo::Drink::new(64, "beer"),
    };
    println!("{meal:?}");
}

#[derive(Debug)]
struct Rec<'a> {
    f1: i32,
    f2: String,
    f3: &'a str,
}

pub fn struct_update_demo() {
    let rec1 = Rec {
        f1: 100,
        f2: String::from("Warm"),
        f3: "Weather",
    };
    println!("{rec1:?}");
    let rec2 = Rec {
        f1: 105,
        f2: String::from("Hot"),
        ..rec1
    };
    println!("{rec2:?}");
    let rec3 = Rec {
        f3: "Climate",
        ..rec1
    };
    println!("{rec3:?}");
    // println!("{rec1:?}"); ⑧
}

#[derive(Debug)]
struct Foo {
    fighter: &'static str,
    number: fn() -> i32,
}

fn foofoo() -> Foo {
    Foo {
        fighter: "Foo Fighters",
        number: || 21,
    }
}

pub fn field_access_demo() {
    let ff1 = foofoo().fighter;
    println!("{ff1}");
    let ff2 = (Foo {
        fighter: "Bar Fighters",
        number: || 42,
    })
    .fighter;
    println!("{ff2}");
    let mut foo = Foo {
        fighter: "Kung Foo",
        number: || 84,
    };
    foo.fighter = "Fung Foo Panda";
    println!("{foo:?}");
    let num = (foo.number)();
    println!("{num}");
}

struct NamedTuple(u16, i32, f64);
struct AnonymousStruct<'a>(bool, &'a str);
struct TuplelikeStruct(u16, i32, f64);

#[derive(Debug)]
struct Triple(u8, u8, i128);
#[derive(Debug)]
struct Pair<T1: Clone, T2: ?Sized>(T1, T2);
pub fn tuple_struct_demo() {
    let ts1 = Triple(0, 10, 100_000_000);
    println!("{ts1:?}");
    let ts2 = Pair("Hi", 007);
    println!("{ts2:?}");
}

pub fn tuple_struct_demo_2() {
    let triple = Triple {
        0: 1,
        1: 10,
        2: 1_000_000_000_000_000_000_000,
    };
    println!("{triple:?}");
    let triple_again = Triple { 1: 255, ..triple };
    println!("{triple_again:?}");
}

#[derive(Debug)]
struct Stock<T: Copy>(&'static str, T);
type Alias = Stock<f32>;
pub fn type_alias_demo() {
    let stock: Alias = Stock("Rust", 1.0);
    println!("{stock:?}");
    // let stock2 = Alias("Iron", 0.0); ③
    // println!("{stock2:?}");
}

#[derive(Debug, PartialEq)]
struct Identity(u64);

impl Identity {
    fn is_valid(&self) -> bool {
        if self.0 >= 1_000_000 && self.0 < 10_000_000 {
            true
        } else {
            false
        }
    }
}
use std::fmt::{Display, Formatter, Result};
impl Display for Identity {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{} ({})", self.0, self.is_valid())
    }
}

fn can_i_pass(id: &Identity) {
    if id.is_valid() {
        println!("You can pass");
    } else {
        println!("You shall not pass!");
    }
}

pub fn newtype_pattern_demo() {
    let id1 = Identity(1234567);
    let id2 = Identity(1);
    println!("{id1}, {id2}");
    assert_ne!(id1, id2);
    for id in &[id1, id2] {
        can_i_pass(id);
    }
    // can_i_pass(123456789);
}

pub fn newtype_pattern_demo_2() {
    for id in &[Identity(7654321), Identity(1000000)] {
        if let Identity(1000000) = id {
            println!("You are wanted! RUN!!!");
        } else {
            let Identity(u1) = id;
            println!("Your internal ID = {u1}");
        }
    }
}

#[derive(Debug, PartialEq)]
struct UnitCircle;
pub fn unit_struct_demo_1() {
    println!("{UnitCircle:?}");
    let circle1 = UnitCircle;
    let circle2 = UnitCircle {};
    assert_eq!(circle1, circle2);
}

impl UnitCircle {
    fn radius() -> f32 {
        1.0
    }
    fn diameter(&self) -> f32 {
        2. * UnitCircle::radius()
    }
}
pub fn unit_struct_demo_2() {
    let r = UnitCircle::radius();
    println!("radius = {r}");
    let d = UnitCircle.diameter();
    println!("diameter = {d}");
}
