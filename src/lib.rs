use std::collections::HashMap;
use std::thread;
use std::time::Duration;

pub struct Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    pub calculation: T,
    pub value: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
where
    T: Fn(K) -> V,
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    pub fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    pub fn value(&mut self, arg: K) -> V {
        if !self.value.contains_key(&arg) {
            let v = (self.calculation)(arg.clone());
            self.value.insert(arg.clone(), v);
        }
        self.value.get(&arg).unwrap().clone()
    }
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure.value(intensity));
        println!("Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cacher_u32() {
        let mut cacher = Cacher::new(|x| x * 2);
        let value = cacher.value(3);
        assert_eq!(value, 6);
    }
    #[test]
    fn test_cacher_str() {
        let mut cacher = Cacher::new(|x: &str| x.len());

        let length = cacher.value("hello");
        assert_eq!(length, 5);
    }
}
