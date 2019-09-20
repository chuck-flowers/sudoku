use crate::grid::iter::IndexIter;
use crate::grid::Coordinate;
use crate::Grid;
use std::convert::TryFrom;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
pub enum ParseGridError {
    OpenFileError,
    MissingRowError,
    ReadError { err: Box<std::error::Error> },
}

pub fn parse_grid(path: impl AsRef<std::path::Path>) -> Result<Grid, ParseGridError> {
    let mut grid = Grid::new();

    let mut reader = if let Ok(reader) = File::open(&path).map(BufReader::new) {
        reader
    } else {
        return Err(ParseGridError::OpenFileError);
    };

    let mut buffer = String::new();

    for row in IndexIter::new() {
        /* Performs the read */
        let read_count = reader
            .read_line(&mut buffer)
            .map_err(|e| ParseGridError::ReadError { err: Box::new(e) })?;

        /* Test if the read returned any data */
        if read_count == 0 {
            return Err(ParseGridError::MissingRowError);
        }

        for (col, c) in IndexIter::new().zip(buffer.chars()) {
            if let Ok(value) = crate::grid::SudokuValue::try_from(c) {
                let coord = Coordinate::new(row, col);
                grid[&coord] = Some(value);
            } else {
                // TODO: Throw an error
            }
        }

        /* Clear the buffer */
        buffer.clear();
    }

    Ok(grid)
}
