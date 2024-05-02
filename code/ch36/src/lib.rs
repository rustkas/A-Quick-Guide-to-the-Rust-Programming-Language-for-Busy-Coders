#[test]
fn ex01() {
    fn vec_iteration_demo() {
        let mut v = vec![1, 2, 3];

        v.clone().into_iter().for_each(|i| println!("{}", i));
        v.iter().for_each(|j| println!("{}", *j));
        v.iter_mut().for_each(|k| {
            *k += 1;
            println!("{}", *k)
        });
    }
    vec_iteration_demo();
}

#[test]
fn ex02() {
    fn vec_iteration_demo() {
        let mut v = vec![1, 2, 3];
        for i in v.clone() {
            println!("{}", i);
        }
        for j in &v {
            println!("{}", *j);
        }
        for k in &mut v {
            *k += 1;
            println!("{}", *k);
        }
    }
    vec_iteration_demo();
}
