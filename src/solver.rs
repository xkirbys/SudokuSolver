use std::io::{stdout, Write};

// Validate the Grid
fn valid(grid: &[[Option<u8>; 9]; 9], row: usize, col: usize, num: u8) -> bool {
    for x in 0..9 {
        if grid[row][x] == Some(num) {
            return false;
        }
    }

    for (x, _item) in grid.iter().enumerate().take(9) {
        if grid[x][col] == Some(num) {
            return false;
        }
    }

    let start_row = row - row % 3;
    let start_col = col - col % 3;

    for x in 0..3 {
        for y in 0..3 {
            if grid[x + start_row][y + start_col] == Some(num) {
                return false;
            }
        }
    }

    true
}

// Solve Sudoku
pub fn solve_sudoku(grid: &mut [[Option<u8>; 9]; 9], progress: &mut f32) -> bool {
    let mut row = 0;
    let mut col = 0;
    let mut check = false;

    let total_cells = 81;
    let mut filled_cells = 0;

    for (x, _item) in grid.iter().enumerate().take(9) {
        for y in 0..9 {
            if grid[x][y].is_some() {
                filled_cells += 1;
            }

            if grid[x][y].is_none() {
                row = x;
                col = y;
                check = true;
                break;
            }
        }
        if check {
            break;
        }
    }

    *progress = filled_cells as f32 / total_cells as f32;


    if !check {
        return true;
    }

    for num in 1..=9 {
        if valid(grid, row, col, num) {
            print!("\r{:.2}% - GUESS: {:?}, CURRENT CELL: {:?}, {:?}", *(progress) * 100.0, num, row, col);
            stdout().flush().expect("TODO: panic message");
            grid[row][col] = Some(num);
            if solve_sudoku(grid, progress) {
                return true;
            }
            grid[row][col] = None;
        }
    }

    false
}
