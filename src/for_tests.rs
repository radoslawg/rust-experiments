pub fn return_me_two() -> i32 {
    return 2;
}

pub struct Rectangle {
    pub width: u8,
    pub height: u8,
}

impl Rectangle {
    pub fn is_square(&self) -> bool {
        self.height == self.width
    }
}
