#[derive(Debug, Copy, Clone)]
enum Piece {
    X,
    O,
}

#[derive(Debug, Copy, Clone)]
enum BoardCell {
    Empty,
    Filled(Piece),
}

impl BoardCell {
    fn get_display_str(&self) -> &'static str {
        match self {
            BoardCell::Empty => " _ ",
            BoardCell::Filled(Piece::X) => " X ",
            BoardCell::Filled(Piece::O) => " O ",
        }
    }
}

#[derive(Debug)]
struct Board {
    board: [[BoardCell; 3]; 3],
}

impl Board {
    fn new() -> Board {
        let board = [[BoardCell::Empty; 3]; 3];
        Board { board }
    }

    fn display(&self) {
        for row in &self.board {
            for c in row {
                print!("{}", c.get_display_str());
            }
            println!();
        }
        println!("-----------");
    }

    fn make_move(&mut self, point: (usize, usize), piece: Piece) {
        self.board[point.0][point.1] = BoardCell::Filled(piece);
    }
}

fn main() {
    let mut board = Board::new();
}
