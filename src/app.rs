use crate::board::Coordinate;

use super::cell::Cell;
use super::game::Game;

pub struct App {
    pub title: String,
    pub should_quit: bool,
    pub game: Game,
    active_column: usize,
    active_row: usize,
}

const SIZE: usize = 3;

impl App {
    pub fn new(title: String) -> Self {
        App {
            title,
            should_quit: false,
            game: Game::new(),
            active_column: 0,
            active_row: 0,
        }
    }

    pub fn on_up(&mut self) {
        if let Some(active_row) = self.active_row.checked_sub(1) {
            self.active_row = active_row
        }
    }

    pub fn on_down(&mut self) {
        if self.active_row < SIZE - 1 {
            self.active_row += 1;
        }
    }

    pub fn on_left(&mut self) {
        if let Some(active_column) = self.active_column.checked_sub(1) {
            self.active_column = active_column;
        }
    }

    pub fn on_right(&mut self) {
        if self.active_column < SIZE - 1 {
            self.active_column += 1;
        }
    }

    pub fn make_move(&mut self) {
        if !self.game.is_ended {
            self.game.make_move(self.get_active_coordinate())
            
        }
        if self.game.game_ended() {
            self.game.is_ended = true;
        }
    }

    pub fn reset_game(&mut self) {
        self.game.reset_game()
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            'q' => {
                self.should_quit = true;
            },
            'r' => {
                self.reset_game()
            }
            _ => {}
        }
    }

    pub fn get_active_coordinate(&self) -> Coordinate {
        (self.active_row, self.active_column)
    }

    pub fn cell(&self, coordinate: Coordinate) -> Cell {
        Cell::new(coordinate)
    }
}
