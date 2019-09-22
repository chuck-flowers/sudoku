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

struct ProgressBar {
    total: usize,
    curr: usize,
}

impl ProgressBar {
    fn new(total: usize) -> Self {
        Self { total, curr: 0 }
    }

    fn update_progress(&mut self, curr: usize) {
        self.curr = curr;
        self.render();
    }

    fn increment(&mut self) {
        self.update_progress(self.curr + 1);
    }

    fn decrement(&mut self) {
        self.update_progress(self.curr - 1);
    }

    fn render(&self) {
        let mut buff = String::new();
        buff.push_str("\r[");
        for _ in 0..self.curr {
            buff.push('=');
        }

        for _ in 0..(self.total - self.curr) {
            buff.push(' ');
        }

        buff.push(']');
        print!("{} {}/{}", buff, self.curr, self.total);
    }
}

fn main() -> Result<(), MainError> {
    let path_buf = build_path();
    let mut grid = io::parse_grid(&path_buf).unwrap();
    println!("Loaded the puzzle:\n{}", grid);

    let mut history = Vec::new();
    let mut empty_slots = grid.all_empty_slots();
    let mut progress_bar = ProgressBar::new(empty_slots.len());

    // Continue altering the grid until a correct solution has been reached.
    let start = SystemTime::now();
    while let Some(empty_slot) = empty_slots.pop() {
        progress_bar.increment();

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
                    progress_bar.decrement();
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
