#![allow(dead_code)]

#[derive(Debug)]
enum PetA {
    Puppy,
    Kitty,
}
pub fn unit_variants() {
    let pet1 = PetA::Puppy;
    let pet2 = PetA::Kitty;
    println!("My pets = {pet1:?} and {pet2:?}");
}

enum PetB {
    Puppy(&'static str),
    Kitty(u8),
}

pub fn tuple_variants() {
    let _p1 = PetB::Puppy("Snoopy");
    let _p2 = PetB::Kitty(9u8);
}

enum PetC {
    Puppy { name: &'static str },
    Kitty { lives: u8 },
}
fn struct_variants() {
    use PetC::*;
    let _p1 = Puppy { name: "Doo" };
    let _p2 = Kitty { lives: 99 };
}

enum Musketeer {
    Athos,
    Aramis = 42,
    Porthos,
}
pub fn enum_discriminants() {
    let m1 = Musketeer::Athos;
    let m2 = Musketeer::Aramis;
    let m3 = Musketeer::Porthos;
    assert_eq!(0, m1 as u8);
    assert_eq!(42, m2 as i16);
    assert_eq!(43, m3 as u32);
}

pub enum Zero {}
pub fn zero_variant_enums_1() -> Zero {
    loop {
        println!("It's not illegal!");
    }
}
pub fn zero_variant_enums() {
    let _z: Zero = panic!("Don't panic!");
}

pub enum Ghost {}
impl Ghost {
    fn boo() {
        println!("I boo!")
    }
    fn doo() {
        println!("I doo!")
    }
}
pub fn another_zero_variant_enum_demo() {
    Ghost::boo();
    Ghost::doo();
}

pub fn non_exostive_demo() {
    #[non_exhaustive]
    pub enum MyEnum {
        Variant1,
        Variant2,
    }
    let my_enum_value = MyEnum::Variant1;
    // Использование перечисления
    let value = match my_enum_value {
        MyEnum::Variant1 => "Variant 1",
        MyEnum::Variant2 => "Variant 2",
        // Поскольку перечисление помечено #[non_exhaustive], нет необходимости обрабатывать все возможные варианты.
    };
    println!("{value}")
}
