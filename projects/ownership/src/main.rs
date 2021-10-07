fn main() {
    let x: String = String::from("hello");
    let mut y: String = x; // move data from x to y
    let z: String = y.clone(); // make a deep copy of y
                               //    println!("{}", x); // compile time error
    y.push_str(", world!");
    println!("{}", y);
    heap_ownership(y); // heap_ownership takes ownership over y but does not pass it back
                       //println!("{}", y); // this will cause compile time error

    let z: String = take_and_return_ownership(z);
    println!("Back in main, z has value: {}", z);

    let (z, len) = take_and_return_tuple(z);
    println!("Back in main, z has value: {}, len is: {}", z, len);
}

fn heap_ownership(x: String) {
    println!("heap_ownership now has ownership over this variable: {}", x);
}

fn take_and_return_ownership(x: String) -> String {
    println!(
        "take_and_return_ownership will take ownership of this variable but return it back: {}",
        x
    );
    x
}

fn take_and_return_tuple(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String
    (s, length)
}
