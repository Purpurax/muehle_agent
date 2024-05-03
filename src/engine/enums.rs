use ggez::graphics::Image;



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

#[derive(Debug)]
pub struct LoadGameError {
    pub message: String,
}
impl LoadGameError {
    pub fn new(message: String) -> LoadGameError {
        LoadGameError {
            message
        }
    }
    pub fn empty() -> LoadGameError {
        LoadGameError { message: "".to_string() }
    }
}

#[derive(Clone)]
pub struct CarryPiece {
    pub x: usize,
    pub ring: usize,
    pub color: Piece,
    pub image: Image
}
impl CarryPiece {
    pub fn new(x: usize, ring: usize, color: Piece, image: Image) -> CarryPiece {
        CarryPiece {x, ring, color, image}
    }
}
impl Into<(usize, usize, Piece, Image)> for CarryPiece {
    fn into(self) -> (usize, usize, Piece, Image) {
        (self.x, self.ring, self.color, self.image)
    }
}


#[derive(Clone, Copy, PartialEq, Debug, Hash, Eq)]
pub enum Piece {
    None,
    White,
    Black
}
impl Piece {
    pub fn to_str(self) -> String {
        if self == Piece::White {
            "White"
        } else if self == Piece::Black {
            "Black"
        } else {
            "None"
        }.to_string()
    }
    
    pub fn parse(string: String) -> Piece {
        if string == "White" {
            Piece::White
        } else if string =="Black" {
            Piece::Black
        } else{
            Piece::None
        }
    }

    /// Converts:
    ///  - White <-> Black
    ///  - None <-> None
    pub fn neg(self) -> Piece {
        if self == Piece::White {
            Piece::Black
        } else if self == Piece::Black {
            Piece::White
        } else {
            Piece::None
        }
    }
}


#[derive(Clone, Copy, PartialEq, Debug)]
pub enum State {
    Setup,
    Normal,
    Take,
    End,
    Win
}
impl State {
    pub fn to_str(self) -> String {
        match self {
            State::Setup => "Setup",
            State::Take => "Take",
            State::End => "End",
            State::Win => "Win",
            _ => "Normal",
        }.to_string()
    }
    pub fn parse(string: &str) -> State {
        match string {
            "Setup" => State::Setup,
            "Take" => State::Take,
            "End" => State::End,
            "Win" => State::Win,
            _ => State::Normal
        }
    }
}