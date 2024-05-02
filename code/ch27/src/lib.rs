#![allow(dead_code)]

#[test]
fn ex01() {
    struct Man;
    struct Boy;
    trait Hombre {
        fn age(&self) -> u8;
    }
    impl Hombre for Man {
        fn age(&self) -> u8 {
            70
        }
    }
    impl Hombre for Boy {
        fn age(&self) -> u8 {
            7
        }
    }
    fn get_age(m: Box<dyn Hombre>) -> u8 {
        m.age()
    }

    fn dyn_trait_demo() {
        let age = get_age(Box::new(Man));
        println!("Man's age = {age}");
        let age = get_age(Box::new(Boy));
        println!("Boy's age = {age}");
    }
    dyn_trait_demo();
}

#[test]
fn ex02() {
    trait MyFirstTrait {}
    trait MySecondTrait {}
    struct One {}
    impl MySecondTrait for One {}
    // fn my_function(arg: impl MyFirstTrait) -> impl MySecondTrait {
    //   todo!();
    //   }

    trait MyTrait {}
    {
        fn my_fn<T: MyTrait>(_arg: T) {}
    }

    {
        fn my_fn(_arg: impl MyTrait) {}
    }
}

#[test]
fn ex03() {
    use std::fmt::Debug;
    pub trait TOne: Copy + Debug {}
    impl TOne for i32 {}
    impl TOne for bool {}
    pub fn func_one(b: bool) -> impl TOne {
        if b {
            42
        } else {
            0
        }
    }

    fn impl_trait_return_demo() {
        let r = func_one(true);
        println!("{:?}", r);
    }
    impl_trait_return_demo();
}

#[test]
fn ex04() {
    pub trait TTwo {
        fn huh(&self) -> u8 {
            111
        }
    }
    impl TTwo for i32 {}
    impl TTwo for bool {}
    pub fn func_two(v: impl TTwo) -> u8 {
        let x = v.huh();
        x
    }

    fn impl_trait_parameter_demo() {
        let val = func_two(true);
        println!("{:?}", val);
        let val = func_two(1_000_000);
        println!("{:?}", val);
    }

    impl_trait_parameter_demo();
}
