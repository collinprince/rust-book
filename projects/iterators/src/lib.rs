#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

struct Counter {
    count: u32,
    upper_bound: u32,
}

impl Counter {
    fn new(upper_bound: u32) -> Counter {
        Counter {
            count: 0,
            upper_bound,
        }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.upper_bound {
            let ret = self.count;
            self.count += 1;
            Some(ret)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn basic_iterator() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        for elem in v1_iter {
            println!("{}", elem);
        }
    }

    #[test]
    fn next_demo() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter(); // creates an iterator over immutable references

        // if we want to create an iterator that takes ownership of v1 and returns owned values, we can
        // call into_iter instead of iter
        // similarly, if we want to iterate over mutable references, we can call iter_mut instead of iter

        assert_eq!(v1_iter.next(), Some(&1)); // next() returns immutable references to the values in the vector
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn mut_iter() {
        let mut v1 = vec![1, 2, 3];

        let v1_iter = v1.iter_mut();

        for elem in v1_iter {
            *elem = *elem + 1;
        }
        assert_eq!(v1, vec![2, 3, 4]);
    }

    #[test]
    fn sum_iter() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        let sum_v1: u32 = v1_iter.sum();

        assert_eq!(sum_v1, 6);
    }

    #[test]
    fn map_iter() {
        let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();

        let v2: Vec<u32> = v1_iter.map(|x| x + 1).collect();

        assert_eq!(v2, vec![2, 3, 4]);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ]
        );
    }

    #[test]
    fn counter_test() {
        let count = Counter::new(3);

        let sum: u32 = count.sum();

        assert_eq!(sum, 3);
    }
}
