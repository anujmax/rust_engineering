use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut fruit = vec!["Apple", "Banana", "Pear", "Peach", "Strawberry"];
    let mut rng = thread_rng();
    fruit.shuffle(&mut rng);
    println!("Fruit salad: ");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
