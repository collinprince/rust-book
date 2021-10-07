fn main() {
    println!("Fibonacci sequence");
    for number in 0..20 {
        println!("{}", fibonacci(number));
    }
}

fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        return 1;
    }
    let mut greater: u32 = 1;
    let mut lesser: u32 = 1;
    for _ in 1..n {
        let temp: u32 = greater + lesser;
        lesser = greater;
        greater = temp;
    }
    greater
}
