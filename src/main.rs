#![warn(clippy::all, clippy::cargo)]

mod actions;
mod grid;
mod io;

use actions::UnappliedAction;
use grid::Grid;
use std::time::SystemTime;

#[derive(Debug)]
pub enum MainError {
    InvalidPartialPuzzle,
    NoSolutionFound,
}

fn main() -> Result<(), MainError> {
    let path_buf = build_path();
    let mut grid = io::parse_grid(&path_buf).unwrap();
    println!("Loaded the puzzle:\n{}", grid);

    let mut history = Vec::new();
    let mut empty_slots = grid.all_empty_slots();
    
    // Continue altering the grid until a correct solution has been reached.
    let start = SystemTime::now();
    while let Some(empty_slot) = empty_slots.pop() {
        
        // Apply the next action, and push it into the history
        let action = UnappliedAction::new(empty_slot);
        history.push(action.apply(&mut grid));

        // Keep iterating on the last applied action in the history while the grid is invalid
        while !grid.is_valid(history.last().unwrap().coordinate()) {
            // While there are still actions in the history, rollback and modify them.
            while let Some(last_action) = history.pop() {
                let mut reverted_action = last_action.revert(&mut grid);

                /* Attempt to modify the reverted action. If a new version of
                 * it is available, reapply. If not, allow for the next action
                 * to be rolled back as well. */
                if reverted_action.increment() {
                    history.push(reverted_action.apply(&mut grid));
                    break;
                } else {
                    let coordinate = reverted_action.into();
                    empty_slots.push(coordinate);
                }
            }
        }
    }

    println!();
    let finish = SystemTime::now();

    if let Ok(time) = finish.duration_since(start) {
        println!("Answer computed in {} seconds", time.as_secs());
    }

    println!("{}", grid);
    Ok(())
}

fn build_path() -> impl AsRef<std::path::Path> {
    let current_dir =
        std::env::current_dir().expect("There was a problem getting the current directory");
    let mut path_buf = current_dir.to_path_buf();
    path_buf.push("sudoku.sdku");
    path_buf
}
