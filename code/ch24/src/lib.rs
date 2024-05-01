#![allow(dead_code)]
#[test]
fn ex01() {
    trait MyTraitOne {
        const C1: i32 = 0b1000_000_000;
        const C2: &'static str;
        type T1;
        type T2<S>;
        fn f1() -> u8 {
            255
        }
        fn f2(a: i64, b: i64) -> Option<i64>;
    }

    // let a: MyTraitOne;

    trait Drone<T: Copy> {
        type Wing<K: Copy>;
        fn new() -> Self;
        fn fly(&self, speed: f32) -> T;
        fn test(&self) -> Result<Self::Wing<T>, ()>;
    }
    // let a: Drone<i32>;
}

#[test]
fn ex02() {
    trait Decapod {
        fn legs(&self) -> u8 {
            10
        }
    }

    trait Crustacean: Decapod {
        fn walk_how(&self) -> &'static str;
    }
    pub fn walk_with_legs<T: Crustacean>(d: T) {
        let how = d.walk_how();
        let legs = d.legs();
        println!("I walk {how} with {legs} legs!");
    }

    struct Crab;
    impl Decapod for Crab {}
    impl Crustacean for Crab {
        fn walk_how(&self) -> &'static str {
            "sideways"
        }
    }
    fn supertraits_demo() {
        let crab = Crab;
        walk_with_legs(crab);
    }
    supertraits_demo();
}
