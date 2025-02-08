mod regex;
mod random;
mod strings;
mod funwithmodules;

fn main() {
    println!("Hello, world!");
    random::run();
    regex::run();
    strings::run();
    funwithmodules::outer_module::print_message();
    funwithmodules::outer_module::inner_module::print_message();
}
