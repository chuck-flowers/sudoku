pub mod coordinate;
pub mod error;
pub mod index;
pub mod iter;

pub use coordinate::Coordinate;
use coordinate::SquareCoordinate;
use index::SudokuIndex;
use iter::RowIter;
use iter::ColIter;
use iter::SquareIter;
use std::fmt::Display;
use std::{collections::HashSet, convert::TryFrom, ops::Deref};

/// The value that can be contained within a SudokuCell.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SudokuValue(u8);

impl SudokuValue {
    const MAX_VALUE: u8 = 9;

    pub fn one() -> Self {
        SudokuValue(1)
    }

    pub fn inc(&mut self) -> bool {
        if self.0 < Self::MAX_VALUE {
            self.0 += 1;
            true
        } else {
            false
        }
    }
}

impl Deref for SudokuValue {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for SudokuValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<char> for SudokuValue {
    type Error = error::SudokuValueConversionError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if let Some(int_val) = value.to_digit(10) {
            if int_val > 0 && int_val < 10 {
                Ok(SudokuValue(int_val as u8))
            } else {
                Err(Self::Error {})
            }
        } else {
            Err(Self::Error {})
        }
    }
}

/// A cell in a Sudoku grid that may or may not be occupied.
pub type SudokuCell = Option<SudokuValue>;

/// A row of SudokuCells.
type Row = [SudokuCell; SudokuIndex::INDEX_MAX];

pub struct Grid {
    rows: [Row; SudokuIndex::INDEX_MAX],
}

impl Grid {
    /// Constructs a new empty grid
    pub fn new() -> Grid {
        let rows = [[None; SudokuIndex::INDEX_MAX]; SudokuIndex::INDEX_MAX];
        Grid { rows }
    }

    /// Returns all the slots that are currently unoccupied
    pub fn all_empty_slots(&self) -> Vec<Coordinate> {
        Coordinate::all_coordinates()
            .filter(|c| self[c].is_none())
            .collect()
    }

    /// Confirms that the grid is in a valid state.
    pub fn is_valid(&self, dirty_coordinate: &Coordinate) -> bool {
        if !self.row_is_valid(dirty_coordinate.row()) {
            false
        } else if !self.col_is_valid(dirty_coordinate.col()) {
            false
        } else if !self.square_is_valid(SquareCoordinate::from(dirty_coordinate)) {
            false
        } else {
            true
        }
    }

    /// Confirms that the row rules are in a valid state
    fn row_is_valid(&self, dirty_row: &SudokuIndex) -> bool {
        !self.contains_duplicate(&mut RowIter::new(*dirty_row))
    }

    /// Confirms the column rules are in a valid state
    fn col_is_valid(&self, dirty_col: &SudokuIndex) -> bool {
        !self.contains_duplicate(&mut ColIter::new(*dirty_col))
    }

    /// Confirms the square rules are in a valid state
    fn square_is_valid(&self, dirty_square: SquareCoordinate) -> bool {
        let square = SquareIter::new(*dirty_square.x(), *dirty_square.y());
        !self.contains_duplicate(square)
    }

    /// Determines whether or not a range contains a duplicate value
    fn contains_duplicate(&self, range: impl Iterator<Item = Coordinate>) -> bool {
        Self::contains_duplicate_value(range.filter_map(|c| self[&c]))
    }

    fn contains_duplicate_value(values: impl Iterator<Item = SudokuValue>) -> bool {
        let mut found_vals = HashSet::new();
        for val in values {
            if !found_vals.insert(val) {
                return true;
            }
        }

        false
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for (i, row) in self.rows.iter().enumerate() {
            if i % 3 == 0 {
                writeln!(f, "|---|---|---|")?;
            }

            for (j, c) in row.iter().enumerate() {
                if j % 3 == 0 {
                    write!(f, "|")?;
                }

                if let Some(val) = c {
                    write!(f, "{}", val)?;
                } else {
                    write!(f, " ")?;
                }
            }

            writeln!(f, "|")?;
        }

        writeln!(f, "|---|---|---|")
    }
}

impl std::ops::Index<&Coordinate> for Grid {
    type Output = Option<SudokuValue>;
    fn index(&self, coordinate: &Coordinate) -> &Self::Output {
        &self.rows[**coordinate.row()][**coordinate.col()]
    }
}

impl std::ops::IndexMut<&Coordinate> for Grid {
    fn index_mut(&mut self, coordinate: &Coordinate) -> &mut Self::Output {
        &mut self.rows[**coordinate.row()][**coordinate.col()]
    }
}
