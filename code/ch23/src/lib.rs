#![allow(dead_code)]

#[test]
fn ex01() {
    #[derive(Debug, Clone)]
    struct Jack {
        clown: Option<Box<Jack>>,
        age: u8,
    }
    impl Jack {
        fn new() -> Jack {
            Jack {
                clown: None,
                age: 1,
            }
        }
        fn push_one(&mut self) {
            self.clown = Some(Box::new(Jack {
                clown: self.clown.clone(),
                age: self.age,
            }));
            self.age = self.age + 1;
        }
    }
}

#[test]
fn ex02() {
    #[derive(Debug, Clone)]
    enum Node<T: Clone + Default> {
        Cons(T, Box<Node<T>>),
        Nil,
    }
}

#[test]
fn ex03() {
    use std::rc::Rc;
    #[derive(Debug)]
    struct Poly(u8);
    fn count(arg: &Rc<Poly>) -> usize {
        Rc::strong_count(&arg)
    }
    pub fn rc_demo() {
        let rc1 = Rc::new(Poly(1));
        let rc2 = rc1.clone();
        let rc3 = rc2.clone();
        println!("{c}", c = count(&rc3));
    }

    rc_demo();
}

#[test]
fn ex04() {
    pub fn cell_demo() {
        let celled = std::cell::Cell::new(10);
        println!("{celled:?}");
        celled.set(1000);
        println!("{celled:?}");
    }

    cell_demo();
}

#[test]
fn ex05() {
    use std::cell::RefCell;
    fn refcell_demo() {
        let mut c = RefCell::new(1);
        *c.get_mut() += 1;
        assert_eq!(c, RefCell::new(2));
        let x = c.replace(10);
        assert_eq!(x, 2);
        assert_eq!(c, RefCell::new(10));
    }
    refcell_demo();
}
