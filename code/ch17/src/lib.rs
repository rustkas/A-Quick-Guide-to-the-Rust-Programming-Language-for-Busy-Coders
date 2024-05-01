use std::fmt::Display;

#[derive(Debug)]
pub struct AnArray<'a, T: Copy, const N: usize>(pub [&'a T; N]);

trait TheArray {
    type Item: Copy;
    fn get(&self, i: usize) -> Option<Self::Item>;
}
impl<'a, T: Copy, const N: usize> TheArray for AnArray<'a, T, N> {
    type Item = T;
    fn get(&self, i: usize) -> Option<Self::Item> {
        if i < N {
            Some(*self.0[i])
        } else {
            None
        }
    }
}
pub fn const_generics_demo() {
    let a = AnArray::<'static, char, 3>([&'a', &'b', &'c']);
    for i in [2, 10] {
        if let Some(e) = a.get(i) {
            println!("Element at {i} = {e}");
        } else {
            println!("No element at index {i}");
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cell<'a, T>
where
    T: Clone + 'a,
{
    pub nucleus: T,
    pub mitochondria: &'a T,
}

pub fn replicate<'a, T>(cell: Cell<'a, T>) -> Cell<'a, T>
where
    T: Clone + 'a,
{
    Cell {
        nucleus: cell.nucleus.clone(),
        mitochondria: cell.mitochondria,
    }
}

trait LiveForever<'a>
where
    Self::LifeForm: 'a + Copy + Display + From<&'a Self::LifeForm>,
{
    type LifeForm;
    fn make_life(creature: &'a Self::LifeForm) -> Self::LifeForm;
}

pub enum Food<T>
where
    T: Clone + Display,
{
    Vegetable(T),
}

trait Human {}
trait Baby: Human {
    fn smile(&self);
}

trait Train<'a> {
    type Car: 'a + Drop;
    fn add_car(&mut self, car: &'a Self::Car);
}

pub enum Either<'a, T: ?Sized> {
    Heaven,
    Hell(&'a T),
}

#[derive(Debug)]
pub struct RefVal<'a, T>(&'a T)
where
    T: 'a + PartialOrd;

pub fn bigger<'a, 'b, 'c, T>(x: &'a RefVal<'a, T>, y: &'b RefVal<'b, T>) -> &'c RefVal<'c, T>
where
    'a: 'c,
    'b: 'c,
    T: 'c + PartialOrd,
{
    if x.0 >= y.0 {
        x
    } else {
        y
    }
}

pub fn apply_fn<F1, F2>(f1: F1, f2: F2)
where
    F1: for<'a> Fn(&'a mut i32),
    F2: for<'b> Fn(&'b i32),
{
    let mut x = 3;
    f1(&mut x);
    f2(&x);
}

pub fn increment<'a>(x: &'a mut i32) {
    *x += 1;
}

pub fn print<'a>(x: &'a i32) {
    println!("value = {}", *x);
}

pub fn hrtb_demo() {
    apply_fn(increment, print);
}
