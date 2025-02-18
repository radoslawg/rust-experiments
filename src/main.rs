mod enums;
mod external_programs;
mod for_tests;
mod funwithmodules;
mod httprequest;
mod option;
mod random;
mod regex;
mod strings;
#[cfg(test)]
mod tests;

fn main() {
    println!("Hello, world!");
    random::run();
    regex::run();
    strings::run();
    funwithmodules::outer_module::print_message();
    funwithmodules::outer_module::inner_module::print_message();
    option::run();
    httprequest::run();
    enums::run();
    external_programs::run();
}
