use crate::grid::coordinate::Coordinate;
use crate::grid::Grid;
use crate::grid::SudokuValue;

/// An action that has already been applied
pub struct AppliedAction {
    coordinate: Coordinate,
    value: SudokuValue,
}

impl AppliedAction {
    pub fn coordinate(&self) -> &Coordinate {
        &self.coordinate
    }
    
    /// Undoes the action on the grid
    pub fn revert(self, grid: &mut Grid) -> UnappliedAction {
        grid[&self.coordinate] = None;
        UnappliedAction {
            coordinate: self.coordinate,
            value: self.value,
        }
    }
}

/// An action that has not been applied yet
#[derive(Debug)]
pub struct UnappliedAction {
    coordinate: Coordinate,
    value: SudokuValue,
}

impl UnappliedAction {
    pub fn new(coordinate: Coordinate) -> Self {
        Self {
            coordinate,
            value: SudokuValue::one(),
        }
    }

    /// Applies the action to a Grid
    pub fn apply(self, grid: &mut Grid) -> AppliedAction {
        grid[&self.coordinate] = Some(self.value);
        AppliedAction {
            coordinate: self.coordinate,
            value: self.value,
        }
    }

    /// Increments the action
    pub fn increment(&mut self) -> bool {
        self.value.inc()
    }
}

impl Into<Coordinate> for UnappliedAction {
    fn into(self) -> Coordinate {
        self.coordinate
    }
}
