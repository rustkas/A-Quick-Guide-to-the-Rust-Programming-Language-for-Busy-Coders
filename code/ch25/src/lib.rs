#[allow(dead_code)]
#[test]
fn ex01() {
    trait SSec {
        const NUMBER: i128;
        const IS_DUE: bool = false;
    }

    // let a: dyn SSec;
    impl SSecCard {
        const VALID_UNTIL: u16 = 2025;
    }
    struct SSecCard;
    impl SSec for SSecCard {
        const NUMBER: i128 = 123_45_6789;
        const IS_DUE: bool = true;
        // const VALID_UNTIL: u16 = 2025;
    }

    let _a = SSecCard {};

    fn associated_constants() {
        println!("My social security number is {}", SSecCard::NUMBER);
        println!("Is due? {}", SSecCard::IS_DUE);
        println!("Valid until: {}", SSecCard::VALID_UNTIL);
    }
    associated_constants();
}

#[test]
fn ex03() {
    trait Maxi {
        type Elem;
        fn maxi(&self) -> Option<Self::Elem>;
    }

    impl Maxi for &[i32] {
        type Elem = i32;
        fn maxi(&self) -> Option<Self::Elem> {
            self.iter().max().copied()
        }
    }

    impl Maxi for &str {
        type Elem = char;
        fn maxi(&self) -> Option<Self::Elem> {
            self.chars().max()
        }
    }

    fn associated_types() {
        let s1: &[i32] = &[21, 31, -11];
        let m1 = s1.maxi().unwrap();
        println!("m1 = {m1}");
        let s2 = "HeLlO, cRaP!";
        let m2 = s2.maxi().unwrap();
        println!("m2 = {m2}");
    }

    associated_types();
}

#[test]
fn ex04() {
    struct Gold {
        karat: u8,
    }
    trait Digger {
        fn new() -> Self;
        fn lucky() -> bool {
            true
        }
    }

    impl Gold {
        fn new(karat: u8) -> Gold {
            Gold { karat }
        }
    }

    impl Digger for Gold {
        fn new() -> Self {
            Gold::new(24)
        }
        // fn lucky() -> bool { false } ②
    }

    fn associated_functions() {
        let gold1 = Gold::new(14);
        let gold2 = <Gold as Digger>::new();
        println!("Gold1 = {}k", gold1.karat);
        println!("Gold2 = {}k", gold2.karat);
        println!(
            "Are you feeling lucky? {}!",
            if Gold::lucky() {
                "Yes, very much"
            } else {
                "No, not at all"
            }
        );
    }
    associated_functions();
}

#[test]
fn ex05() {
    struct Burrito<T: Copy + Default>(T);

    trait Wrap {
        type Core: Copy + Default;
        fn unwrap(&self) -> Self::Core {
            Self::Core::default()
        }
        fn rewrap(&mut self) -> Self;
    }
    impl<T: Copy + Default> Burrito<T> {
        fn new(core: T) -> Self {
            Burrito::<T>(core)
        }
    }

    impl<T: Copy + Default> Wrap for Burrito<T> {
        type Core = T;
        fn unwrap(&self) -> Self::Core {
            self.0
        }
        fn rewrap(&mut self) -> Burrito<T> {
            let rewrapped = Burrito::<T>::new(self.0);
            self.0 = T::default();
            rewrapped
        }
    }

    fn associated_methods() {
        let mut b1 = Burrito::new(42);
        let u1 = b1.unwrap();
        assert_eq!(42, u1);
        let b2 = b1.rewrap();
        assert_eq!(42, b2.unwrap());
        assert_eq!(0, b1.unwrap());
    }

    associated_methods();
}
