mod error;

use std::convert::TryFrom;
use std::fmt::Display;
use std::ops::Deref;

type IndexValue = usize;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct SudokuIndex(IndexValue);

impl SudokuIndex {
    /// The exclusive maximum for an index
    pub const INDEX_MAX: IndexValue = 9;

    /// Creates a zero index
    pub fn zero() -> Self {
        Self(0)
    }

    /// Attempt to increment to the next index value. Returns true if it was
    /// incremented successfully.
    pub fn inc(&mut self, amount: IndexValue) -> bool {
        if self.0 + amount < Self::INDEX_MAX {
            self.0 += amount;
            true
        } else {
            false
        }
    }
}

impl Deref for SudokuIndex {
    type Target = IndexValue;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for SudokuIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<IndexValue> for SudokuIndex {
    type Error = error::IndexConversionError;

    fn try_from(val: IndexValue) -> Result<SudokuIndex, Self::Error> {
        if (0..=Self::INDEX_MAX).contains(&val) {
            Ok(SudokuIndex(val))
        } else {
            Err(Self::Error {})
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SquareIndex(IndexValue);

impl SquareIndex {
    const INDEX_MAX: IndexValue = 2;

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn inc(&mut self) -> bool {
        if self.0 < Self::INDEX_MAX {
            self.0 += 1;
            true
        } else {
            false
        }
    }

    pub fn start(&self) -> SudokuIndex {
        SudokuIndex(self.0 * 3)
    }
}

impl Deref for SquareIndex {
    type Target = IndexValue;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for SquareIndex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<SudokuIndex> for SquareIndex {
    fn from(index: SudokuIndex) -> Self {
        SquareIndex(index.0 / 3)
    }
}

impl TryFrom<IndexValue> for SquareIndex {
    type Error = error::IndexConversionError;
    fn try_from(value: IndexValue) -> Result<Self, Self::Error> {
        if (0..=Self::INDEX_MAX).contains(&value) {
            Ok(SquareIndex(value))
        } else {
            Err(Self::Error {})
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn square_index_from_sudoku_index() {
        struct Arg {
            index_val: usize,
            square_val: usize,
        }

        let args = [
            Arg {
                index_val: 0,
                square_val: 0,
            },
            Arg {
                index_val: 8,
                square_val: 2,
            },
            Arg {
                index_val: 2,
                square_val: 0,
            },
            Arg {
                index_val: 3,
                square_val: 1,
            },
        ];

        for a in args.iter() {
            let sudoku_index = SudokuIndex::try_from(a.index_val).unwrap();
            let actual_square_index: SquareIndex = sudoku_index.into();
            let expected_square_index = SquareIndex::try_from(a.square_val).unwrap();
            assert_eq!(actual_square_index, expected_square_index);
        }
    }

    #[test]
    fn top_left_sudoku_index_from_square_index() {
        struct Arg {
            square_val: usize,
            index_val: usize,
        }

        let args = [
            Arg {
                square_val: 0,
                index_val: 0,
            },
            Arg {
                square_val: 1,
                index_val: 3,
            },
            Arg {
                square_val: 2,
                index_val: 6,
            },
        ];

        for a in args.iter() {
            let square_index = SquareIndex::try_from(a.square_val).unwrap();
            let actual_index = square_index.start();
            let expected_index = SudokuIndex::try_from(a.index_val).unwrap();
            assert_eq!(actual_index, expected_index);
        }
    }
}
