use crate::grid::index::SquareIndex;
use crate::grid::index::SudokuIndex;
use crate::grid::Coordinate;
use std::convert::TryFrom;

pub struct IndexIter {
    next: SudokuIndex,
    has_next: bool,
}

impl IndexIter {
    pub fn new() -> Self {
        IndexIter {
            next: SudokuIndex::zero(),
            has_next: true,
        }
    }
}

impl Iterator for IndexIter {
    type Item = SudokuIndex;
    fn next(&mut self) -> Option<Self::Item> {
        if self.has_next {
            let next = self.next;
            self.has_next = self.next.inc(1);
            Some(next)
        } else {
            None
        }
    }
}

pub struct RowIter {
    row: SudokuIndex,
    iter: IndexIter,
}

impl<'a> RowIter {
    pub fn new(row: SudokuIndex) -> Self {
        RowIter {
            row,
            iter: IndexIter::new(),
        }
    }
}

impl Iterator for RowIter {
    type Item = Coordinate;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(col) = self.iter.next() {
            Some(Coordinate::new(self.row, col))
        } else {
            None
        }
    }
}

/// Iterates over the cells within a column. Iterates from the top of the column to the bottom of the column.
pub struct ColIter {
    col: SudokuIndex,
    iter: IndexIter,
}

impl ColIter {
    pub fn new(col: SudokuIndex) -> Self {
        ColIter {
            col,
            iter: IndexIter::new(),
        }
    }
}

impl Iterator for ColIter {
    type Item = Coordinate;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(row) = self.iter.next() {
            Some(Coordinate::new(row, self.col))
        } else {
            None
        }
    }
}

pub struct SquareIter {
    x: SudokuIndex,
    y: SudokuIndex,
    counter: usize,
}

impl SquareIter {
    pub fn new(x: SquareIndex, y: SquareIndex) -> Self {
        Self {
            x: x.start(),
            y: y.start(),
            counter: 0,
        }
    }
}

impl Iterator for SquareIter {
    type Item = Coordinate;
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter < SudokuIndex::INDEX_MAX {
            let raw_x = *self.x + (self.counter % 3);
            let raw_y = *self.y + (self.counter / 3);
            self.counter += 1;
            let x = SudokuIndex::try_from(raw_x).unwrap();
            let y = SudokuIndex::try_from(raw_y).unwrap();
            Some(Coordinate::new(x, y))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_iter_count_test() {
        assert_eq!(9, IndexIter::new().count());
    }
}
