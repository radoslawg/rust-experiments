pub fn run() {
    let some_string = String::from("Warszawa");
    println!("10th Character of {} is {}", some_string, match some_string.chars().nth(10) {
        Some(s) => s.to_string(),
        None => "No character found".to_string()
    });

    println!("option1: {}", option_pattern("option1").unwrap());
    println!("option2: {}", option_pattern("option2").unwrap());
    println!("option3: {}", match option_pattern("option3") {
        Some(s) => s,
        None => "Error"
    });
    println!("Should print some => {}", Some("o").and_then(option_pattern).and_then(option_pattern).unwrap()); // monad's bind progression
    println!("Should print true => {}", Some("os").and_then(option_pattern).and_then(option_pattern).is_none()); // monad's bind progression

}

fn option_pattern(o: &str) -> Option<&str> {
    match o {
        "option1" => Some("some"),
        "option2" => Some("something else"),
        "o" => Some("option1"),
        _ => None
    }
}