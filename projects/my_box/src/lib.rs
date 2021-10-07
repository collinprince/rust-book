struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deref_works() {
        let x = 5;
        let y = MyBox::new(x);

        assert_eq!(5, x);
        assert_eq!(5, *y);
    }

    #[test]
    fn deref_coercion() {
        fn hello(name: &str) {
            println!("Hello, {}!", name);
        }

        let m = MyBox::new(String::from("Rust"));
        hello(&m);
    }

    #[test]
    fn no_deref_coercion() {
        fn hello(name: &str) {
            println!("Hello, {}!", name);
        }

        let m = MyBox::new(String::from("Rust"));
        hello(&(*m)[..]);
    }
}
