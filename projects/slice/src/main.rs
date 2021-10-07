fn main() {
    let s = String::from("Hello, world!");
    let copy = first_word(&s);
    println!("Copy is: {}", copy);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &elem) in bytes.iter().enumerate() {
        if elem == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
