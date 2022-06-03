use crate::board::Coordinate;

pub struct Cell {
    pub coordinate: Coordinate,
}

impl Cell {
    pub fn new(coordinate: Coordinate) -> Cell {
        Self { coordinate }
    }

    pub fn is_active(&self, active_coordinate: Coordinate) -> bool {
        self.coordinate == active_coordinate
    }
}
