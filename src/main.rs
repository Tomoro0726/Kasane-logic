use std::collections::{HashMap, HashSet};

fn main() {
    // HashSet example
    let mut animals: HashSet<&str> = HashSet::new();
    animals.insert("cat");
    animals.insert("dog");
    animals.insert("cat"); // duplicate ignored
    println!("Set: {:?}", animals);

    // HashMap example
    let mut fruits: HashMap<&str, i32> = HashMap::new();
    fruits.insert("apple", 3);
    fruits.insert("banana", 5);
    println!("Map: {:?}", fruits);

    if let Some(count) = fruits.get("apple") {
        println!("Apples: {}", count);
    }
}
