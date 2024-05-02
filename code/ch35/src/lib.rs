use std::{
    cmp::Ordering,
    fmt::{Display, Formatter},
    ops::Add,
};
#[derive(Debug, Clone, Copy)]
struct Int(i32);
impl Add for Int {
    type Output = Int;
    fn add(self, rhs: Self) -> Self::Output {
        Int(self.0 + rhs.0)
    }
}
impl PartialEq for Int {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl PartialOrd for Int {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}
impl Display for Int {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{d}", d = self.0)
    }
}

#[test]
fn overloading_demo() {
    let sum = Int(1) + Int(2);
    println!("sum = {sum}");
    assert!(Int(10) == Int(10));
    assert!(Int(1) != Int(-1));
    assert!(Int(100) > Int(1));
    assert!(Int(1) < Int(100));
}
