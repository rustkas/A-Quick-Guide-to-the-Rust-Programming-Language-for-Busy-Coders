#![allow(dead_code)]

use serde_derive::*;
// use std::fmt::{Display, Formatter, Result};
#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
struct Car {
    num_wheels: u8,
    is_electric: bool,
}

#[derive(Debug)]
struct Gene<'a, T: Copy> {
    pub dna: &'a [T],
}
impl<'a, T: Copy> Gene<'a, T> {
    pub fn first(gene: &Self) -> Option<T> {
        if let &[f, ..] = gene.dna {
            Some(f)
        } else {
            None
        }
    }
}
pub fn turbofish_demo<'a>() {
    let g = Gene::<'a, u8> { dna: &[1, 2, 3] };
    println!("{g:?}");
    let f = Gene::<'a, u8>::first(&g);
    println!("First element: {f:?}");
}

pub fn crab() {
    println!("Just crab!");
}

pub mod sea {
    pub fn crab() {
        println!("Sea crab!");
    }
    pub mod world {
        pub fn crab() {
            super::super::crab();
            self::super::crab();
        }
    }
}

mod dealer {
    pub use self::deck::deal_one;
    mod deck {
        pub fn deal_one() -> i32 {
            42
        }
    }
}
pub fn pub_use_demo() {
    // let uno = dealer::deck::deal_one(); ④
    let one = dealer::deal_one();
    println!("dealt = {one}");
}
