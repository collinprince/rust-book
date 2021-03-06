enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[cfg(test)]
mod tests {

    use crate::List::{Cons, Nil};
    #[test]
    fn make_cons_list() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }
}
