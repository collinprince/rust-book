struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn drop_test() {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };

        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        println!("CustomSmartPointers created.");
    }

    #[test]
    fn drop_early() {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };

        println!("Created CustomSmartPointer c");
        std::mem::drop(c);
        println!("CustomSmartPointer c dropped before the end of main.");
    }
}
