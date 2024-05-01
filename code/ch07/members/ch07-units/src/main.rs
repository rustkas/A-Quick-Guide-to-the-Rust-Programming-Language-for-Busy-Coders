#![allow(unsafe_code)]

#[repr(C)]
union RiskyUnion {
    are_u: u32,
    press_f: f32,
}

fn union_demo() {
    let u = RiskyUnion { are_u: 1 };
    let f = unsafe { u.are_u };
    println!("{f}");
}

pub enum Diet {
    Keto,
    Paleo(u8, i128),
    Mediterranean(bool),
}

union UnsafeUnion {
    i: i32,
    f: f32,
}
fn union_demo2() {
    let u = UnsafeUnion { i: 100 };
    let f = unsafe { u.f };
    println!("{f}");
}

fn main() {
    union_demo();
    union_demo2();
}
