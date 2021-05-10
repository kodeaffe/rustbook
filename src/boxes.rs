use std::ops::{Deref, DerefMut};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}


fn hello(name: &str) {
    println!("Hello, {}!", name);
}

pub fn run() {
    let x = 5;
    let mut y = MyBox::new(x);

    assert_eq!(5, x);
    *y += 1;
    assert_eq!(6, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}