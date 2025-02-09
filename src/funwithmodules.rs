pub mod outer_module {

    pub mod inner_module {
        pub fn print_message() {
            println!("Hello from the inner side");
        }
    }

    pub fn print_message() {
        println!("Hello from the outer side");
    }
}
