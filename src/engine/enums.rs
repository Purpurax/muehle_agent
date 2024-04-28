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