fn main() {
    // create empty array of ints
    let mut v: Vec<i32> = Vec::new();

    // create array of ints (implied i32 by default)
    let mut v2 = vec![1, 2, 3];

    // pushing to a vector
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    v2.push(4);
    v2.push(5);

    // reading elements of vectors
    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    // use match to read element from vector
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }

    // the &vec[index] method will panic if the index we desire does not exist
    // the vec.get(inex) method will just return none if the index we desire does not exist

    // mutable reference rules are still applied to vectors
    let first: &i32 = &v[0];
    v.push(10);
    // if we uncomment the below line, then there will be an error since we hold onto the immutable
    // ref for first in addition to the mutable reference that is borrowed when we push 10
    // println!("The first element is: {}", first);

    // iterating over the values in a vector
    let v3 = vec![100, 200, 300];
    for i in &v3 {
        println!("{}", i);
    }

    // we can also iterate over mutable references to each element in a mutable vector in order to
    // make changes to all the elements
    let mut v4 = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // using an enum to store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
