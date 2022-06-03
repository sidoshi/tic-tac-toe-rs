use regex::Regex;
use std::io;

use crate::board::{Board, BoardCell, Coordinate, Piece};

pub struct Game {
    board: Board,
    turn: Piece,
    moves: u8,
    is_ended: bool
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            turn: Piece::X,
            moves: 0,
            is_ended: false,
        }
    }

    pub fn start(&mut self) {
        loop {
            self.board.display();
            println!("Turn: {}", self.turn.get_display_str());
            let point = Game::get_point_input();
            self.make_move(point);
        }
    }

    pub fn reset_game(&mut self) {
        self.board = Board::new();
        self.turn = Piece::X;
        self.moves = 0;
        self.is_ended = false;
    }

    fn check_winner(&self) -> Option<Piece> {
        let win_indexes = [
            [(0, 0), (0, 1), (0, 2)],
            [(1, 0), (1, 1), (1, 2)],
            [(2, 0), (2, 1), (2, 2)],
            [(0, 0), (1, 0), (2, 0)],
            [(0, 1), (1, 1), (2, 1)],
            [(0, 2), (1, 2), (2, 2)],
            [(0, 0), (1, 1), (2, 2)],
            [(0, 2), (1, 1), (2, 0)],
        ];

        for i in win_indexes {
            if self.board.get_piece_at(i[0]).is_some()
                && self.board.get_piece_at(i[0]) == self.board.get_piece_at(i[1])
                && self.board.get_piece_at(i[0]) == self.board.get_piece_at(i[2])
            {
                return self.board.get_piece_at(i[0]);
            }
        }

        None
    }

    pub fn get_cell_value(&self, point: Coordinate) -> &str {
        self.board.get_board_cell_value(point)
    }

    pub fn game_ended(&mut self) -> bool {
        if self.is_ended {
            return true
        }
            
        if let Some(p) = self.check_winner() {
            self.is_ended = true;
            println!("Player {} won! {}", p.get_display_str(), '\u{1F378}');
            return true;
        }

        if self.moves == 9 {
            self.is_ended = true;
            println!("Tie!");
            return true;
        }

        false
    }

    fn get_point_input() -> Coordinate {
        loop {
            let mut point = String::new();
            io::stdin()
                .read_line(&mut point)
                .expect("failed to readline");

            let re = Regex::new("[0-2] [0-2]").expect("Failed to create regex");
            if re.is_match(&point) {
                let (x, y) = point.split_at(1);

                return (x.trim().parse().unwrap(), y.trim().parse().unwrap());
            }

            println!("Invalid Input");
        }
    }

    pub fn make_move(&mut self, point: Coordinate) {
        let valid = self.board.is_empty(point);
        if valid {
            self.board.make_move(point, self.turn);
            self.change_turn();
            self.moves = self.moves + 1;
            self.game_ended();
        } else {
            println!("Invalid move");
        }
    }

    fn change_turn(&mut self) {
        self.turn = match self.turn {
            Piece::O => Piece::X,
            Piece::X => Piece::O,
        }
    }
}
