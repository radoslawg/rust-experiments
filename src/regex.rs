extern crate regex;
use regex::Regex;

pub fn run () {
    let re: Regex;
    match Regex::new(r"\w{5}") {
        Ok(regex) => re = regex,
        Err(err) => {println!("There was an error {}", err); return},
    }
    let text = "Radek";
    println!("Found match {}", re.is_match(text))
}