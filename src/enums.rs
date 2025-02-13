enum Day {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl Day {
    fn is_weekday(&self) -> bool {
        match self {
            &Day::Saturday | &Day::Sunday => return false,
            _ => return true,
        }
    }
}

pub fn run() {
    let d = Day::Tuesday;
    println!("Is d a weekday? {}", d.is_weekday());
    let d2 = Day::Saturday;
    println!("Is d a weekday? {}", d2.is_weekday());
}
