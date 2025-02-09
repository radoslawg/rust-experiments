pub fn run() {
    let my_string = String::from("I hate Rust");
    println!("Replacing: {}", my_string.replace("hate", "love"));

    let lines_string: String = String::from("This is\nmultiline\nstring");
    for line in lines_string.lines() {
        println!("[ {} ]", line);
    }

    let split_string: String = String::from("Split+This+String");
    let splitted_vector: Vec<&str> = split_string.split("+").collect(); // Collect is a trait of iterator that changes iterator into collection
    println!("Seocond splitted word in {} is {}", split_string, splitted_vector[1]);

    let trimmed_string = String::from("     String to be trimmed   ");
    println!("Trimmed string: '{}'", trimmed_string.trim());

    let nth_char_string = String::from("Hello from the Rust side");
    match nth_char_string.chars().nth(4) {
        Some(c) => println!("Found 5th character and it is {}", c),
        None => println!("Did not find character at position 4")
    }
    match nth_char_string.chars().nth(40) {
        Some(c) => println!("Found 41st character and it is {}", c),
        None => println!("Did not find character at position 40")
    }

}