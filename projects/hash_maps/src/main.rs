use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores_2: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // use get to fetch value from array by key (reference)
    let score = scores_2.get(&String::from("Blue"));

    if let Some(val) = score {
        println!("Key 'Blue' has value: {}", val);
    }

    for (key, value) in &scores_2 {
        println!("{}: {}", key, value);
    }

    // overwriting a value
    scores.insert(String::from("Blue"), 44);
    println!("{:?}", scores);

    // only inserting a value if the key has no value
    // or_insert: defined to retun a mutable reference to the value for the corresponding Entry
    // key if that key exists, and if not, inserts the parameter as the new value for this
    // key and returns a mutable reference to the new value
    scores.entry(String::from("Yellow")).or_insert(50); // this will insert "Yellow": 50 since it doesn't exist
    scores.entry(String::from("Blue")).or_insert(50); // this will see that "Blue" is already present
                                                      // and the entry will not be updated

    println!("{:?}", scores);

    // Updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
