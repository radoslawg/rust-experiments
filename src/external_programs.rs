use std::process::Command;

pub fn run() {
    let mut cmd = Command::new("ls");
    cmd.arg("-la");

    match cmd.output() {
        Ok(o) => unsafe {
            println!("Output of ls is: {}", String::from_utf8_unchecked(o.stdout));
        },
        Err(_) => {
            println!("Error executing command");
        }
    }
}
