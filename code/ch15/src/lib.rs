trait Diet {
    const FOOD: &'static str;
    fn eat(&self);
}
impl Diet for i32 {
    const FOOD: &'static str = "Grapes";
    fn eat(&self) {
        static MEAL: &'static str = i32::FOOD;
        println!("I only eat {MEAL}");
    }
}
static F: &'static str = "Olive oil";
pub fn statics_demo() {
    println!("{F}");
    static D1: i32 = 42;
    D1.eat();
}

pub fn fun_1() -> i32 {
    ({
        #[inline]
        |x, y| x + y
    })(1, 2)
}

#[inline(always)]
pub fn fun_2() -> f64 {
    std::f64::consts::PI
}

#[inline(never)]
pub fn fun_3(x: bool, y: bool, z: bool) -> bool {
    (x || y) && z
}
