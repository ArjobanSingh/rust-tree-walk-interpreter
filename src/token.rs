#[derive(Debug)]
pub struct Token {
    value: char,
}

impl Token {
    pub fn new(value: char) -> Self {
        Token { value }
    }
}