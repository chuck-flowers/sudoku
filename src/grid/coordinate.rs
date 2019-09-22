use crate::grid::index::SquareIndex;
use crate::grid::index::SudokuIndex;

/// A location on the sudoku grid described by a row and column index.
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Coordinate {
    row: SudokuIndex,
    col: SudokuIndex,
}

impl Coordinate {
    pub fn new(row: SudokuIndex, col: SudokuIndex) -> Coordinate {
        Coordinate { row, col }
    }

    /// Iterates over all possible coordinate values
    pub fn all_coordinates() -> impl Iterator<Item = Self> {
        let mut row = SudokuIndex::zero();
        let mut col = SudokuIndex::zero();
        let mut has_next = true;

        std::iter::from_fn(move || {
            if has_next {
                let coor = Coordinate::new(row, col);
                if !col.inc(1) {
                    col = SudokuIndex::zero();
                    has_next = row.inc(1);
                }

                Some(coor)
            } else {
                None
            }
        })
    }

    pub fn row<'a>(&'a self) -> &'a SudokuIndex {
        &self.row
    }

    pub fn col<'a>(&'a self) -> &'a SudokuIndex {
        &self.col
    }
}

pub struct SquareCoordinate {
    x: SquareIndex,
    y: SquareIndex
}

impl SquareCoordinate {
    pub fn new(x: SquareIndex, y: SquareIndex) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> &SquareIndex {
        &self.x
    }

    pub fn y(&self) -> &SquareIndex {
        &self.y
    }
}

impl From<&Coordinate> for SquareCoordinate {
    fn from(coor: &Coordinate) -> Self {
        let x = coor.row().into();
        let y = coor.col().into();
        SquareCoordinate::new(x, y)
    }
}