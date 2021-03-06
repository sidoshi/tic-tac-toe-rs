#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Piece {
    X,
    O,
}

impl Piece {
    pub fn get_display_str(&self) -> &'static str {
        match self {
            Piece::X => " X ",
            Piece::O => " O ",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BoardCell {
    Empty,
    Filled(Piece),
}

pub type Coordinate = (usize, usize);

impl BoardCell {
    fn get_display_str(&self) -> &'static str {
        match self {
            BoardCell::Empty => " _ ",
            BoardCell::Filled(piece) => piece.get_display_str(),
        }
    }
}

#[derive(Debug)]
pub struct Board {
    board: [[BoardCell; 3]; 3],
}

impl Board {
    pub fn new() -> Board {
        let board = [[BoardCell::Empty; 3]; 3];
        Board { board }
    }

    pub fn display(&self) {
        for row in &self.board {
            for c in row {
                print!("{}", c.get_display_str());
            }
            println!();
        }
        println!("-----------");
    }

    pub fn make_move(&mut self, point: Coordinate, piece: Piece) {
        self.board[point.0][point.1] = BoardCell::Filled(piece);
    }

    pub fn is_empty(&self, point: Coordinate) -> bool {
        self.board[point.0][point.1] == BoardCell::Empty
    }

    pub fn get_board_cell_value(&self, point: Coordinate) -> &str {
        self.board[point.0][point.1].get_display_str()
    }

    pub fn get_piece_at(&self, point: Coordinate) -> Option<Piece> {
        if let BoardCell::Filled(piece) = self.board[point.0][point.1] {
            return Some(piece);
        }
        None
    }
}
