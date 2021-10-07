fn main() {
    println!("Hello, world!");

    another_function(5, 10);

    println!("The value returned by five() is: {}", five());
    println!("The value returned by plus_one(1) is: {}", plus_one(1));
}

fn another_function(x: i32, y: i32) {
    println!("Another function's parameter {} {}!", x, y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
