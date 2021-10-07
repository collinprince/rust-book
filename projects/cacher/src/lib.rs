use std::collections::HashMap;
use std::hash::Hash;

pub struct Cacher<T, P, Q>
where
    P: Hash + Eq,
    Q: Copy,
    T: Fn(P) -> Q,
{
    calculation: T,
    map: HashMap<P, Q>,
}

impl<T, P, Q> Cacher<T, P, Q>
where
    P: Hash + Eq + Copy,
    Q: Copy,
    T: Fn(P) -> Q,
{
    pub fn new(calculation: T) -> Cacher<T, P, Q> {
        Cacher {
            calculation,
            map: HashMap::new(),
        }
    }

    pub fn value(&mut self, input: P) -> Q {
        match self.map.get(&input) {
            Some(val) => *val,
            None => {
                let result = (self.calculation)(input);
                self.map.insert(input, result);
                result
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sanity_test() {
        let mut cacher = Cacher::new(|x| x + 1);
        let result = cacher.value(2);
        assert_eq!(result, 3);
    }

    #[test]
    fn multiple_input_values() {
        let mut cacher = Cacher::new(|x| x + 1);
        let ret1 = cacher.value(2);
        let ret2 = cacher.value(3);
        let ret3 = cacher.value(2);
        assert_eq!(ret1, 3);
        assert_eq!(ret2, 4);
        assert_eq!(ret3, 3);
    }

    #[test]
    fn different_types_cacher() {
        let mut ints_cacher = Cacher::new(|x| x + 1);
        let mut string_to_int_cacher = Cacher::new(|x: &str| x.len() + 1);
        let ret1 = ints_cacher.value(2);
        let ret2 = string_to_int_cacher.value("hello");
        assert_eq!(ret1, 3);
        assert_eq!(ret2, 6);
    }
}
