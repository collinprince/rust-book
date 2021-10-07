fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_ref<T: PartialOrd + Copy>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if *item > *largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let v1 = vec![1, 2, 3, 4, 5];
    println!("The greatest elem is: {}", largest(&v1));
    println!("The greatest elem is: {}", largest(&v1));
}
