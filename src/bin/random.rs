use rand::Rng;

fn main() {
    let mut rng = rand::rng();
    let random_number: u32 = rng.random_range(..11);
    println!("Random number: {}", random_number);
}