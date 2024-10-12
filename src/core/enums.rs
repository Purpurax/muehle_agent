#[derive(Debug)]
pub struct FieldError {
    pub message: String,
}
impl FieldError {
    pub fn new(message: String) -> FieldError {
        FieldError {
            message
        }
    }
    pub fn empty() -> FieldError {
        FieldError { message: "".to_string() }
    }
}

#[derive(Clone)]
pub struct CarryPiece {
    pub position: usize,
    pub color: u8
}
impl CarryPiece {
    pub fn new(position: usize, color: u8) -> CarryPiece {
        CarryPiece {position, color}
    }
}
impl Into<(usize, u8)> for CarryPiece {
    fn into(self) -> (usize, u8) {
        (self.position, self.color)
    }
}



#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    Setup,
    Normal,
    Take,
    Win
}
impl State {
    pub fn to_str(self) -> String {
        match self {
            State::Setup => "Setup",
            State::Take => "Take",
            State::Win => "Win",
            _ => "Normal",
        }.to_string()
    }
    pub fn parse(string: &str) -> State {
        match string {
            "Setup" => State::Setup,
            "Take" => State::Take,
            "Win" => State::Win,
            _ => State::Normal
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Difficulty {
    Off,
    Easy,
    Medium,
    Hard
}