use crate::grid::coordinate::Coordinate;
use crate::grid::Grid;
use crate::grid::SudokuValue;
use std::fmt::Debug;

/// An action that has already been applied
pub struct AppliedAction<A: SudokuAction> {
    action: A,
}

impl<A: SudokuAction> AppliedAction<A> {
    /// Undoes the action on the grid
    pub fn revert(self, grid: &mut Grid) -> UnappliedAction<A> {
        let action = self.action;
        action.revert(grid);

        UnappliedAction { action }
    }
}

/// An action that has not been applied yet
#[derive(Debug)]
pub struct UnappliedAction<A: SudokuAction> {
    action: A,
}

impl<A: SudokuAction> UnappliedAction<A> {
    /// Creates a new UnappliedAction
    pub fn new(action: A) -> Self {
        UnappliedAction { action }
    }

    /// Applies the action to a Grid
    pub fn apply(self, grid: &mut Grid) -> AppliedAction<A> {
        let action = self.action;
        action.apply(grid);
        AppliedAction { action }
    }

    /// Increments the action
    pub fn increment(&mut self) -> bool {
        self.action.increment()
    }

    pub fn into(self) -> A {
        self.action
    }
}

pub trait SudokuAction: Debug {
    fn apply(&self, grid: &mut Grid);
    fn revert(&self, grid: &mut Grid);
    fn increment(&mut self) -> bool;
}

#[derive(Debug)]
pub struct WriteAction {
    coordinate: Coordinate,
    value: SudokuValue,
}

impl Into<Coordinate> for WriteAction {
    fn into(self) -> Coordinate {
        self.coordinate
    }
}

impl WriteAction {
    pub fn new(coordinate: Coordinate) -> Self {
        Self {
            coordinate,
            value: SudokuValue::one(),
        }
    }
}

impl SudokuAction for WriteAction {
    fn apply(&self, grid: &mut Grid) {
        grid[&self.coordinate] = Some(self.value);
    }

    fn revert(&self, grid: &mut Grid) {
        grid[&self.coordinate] = None;
    }

    fn increment(&mut self) -> bool {
        self.value.inc()
    }
}
