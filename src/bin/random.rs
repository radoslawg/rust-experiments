use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    let random_number: u32 = rng.random_range(..11);
    println!("Random number: {}", random_number);
    
    let random_weight: f64 = rng.random();
    let random_bool = rng.random_bool(random_weight);
    println!("Random bool: {} with Random weight: {}", random_bool, random_weight);
}