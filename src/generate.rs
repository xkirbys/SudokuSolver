use reqwest::blocking::{Client, get};
use reqwest::header::CONTENT_TYPE;
use serde_derive::{Deserialize, Serialize};
use serde_json::from_str;
use crate::Difficulty;

#[derive(Deserialize, Debug)]
struct SudokuApi {
    difficulty: String,
    puzzle: String,
    solution: String,
}

#[derive(Serialize, Debug)]
struct SudokuRequest {
    difficulty: String,
    solution: bool,
    array: bool,
}

// Function to generate Sudoku puzzle
pub fn generate_sudoku(difficulty: Difficulty) -> [[Option<u8>; 9]; 9] {
    let client = Client::new();
    let body = SudokuRequest {
        difficulty: difficulty.to_string(),
        solution: true,
        array: false,
    };

    let response = client.post("https://you-do-sudoku-api.vercel.app/api")
        .header(CONTENT_TYPE, "application/json")
        .json(&body)
        .send()
        .expect("Failed to send request")
        .text()
        .expect("Failed to read response");

    let sudoku: SudokuApi = from_str(&response).expect("Failed to deserialize response");

    // Convert the puzzle string into a 2D array
    let puzzle: Vec<Vec<Option<u8>>> = sudoku.puzzle.chars()
        .map(|c| if c == '0' { None } else { Some(c.to_digit(10).unwrap() as u8) })
        .collect::<Vec<Option<u8>>>()
        .chunks(9)
        .map(|chunk| chunk.to_vec())
        .collect();

    // Convert Vec<Vec<Option<u8>>> to [[Option<u8>; 9]; 9]
    let mut puzzle_array = [[None; 9]; 9];
    for (i, row) in puzzle.iter().enumerate() {
        for (j, &item) in row.iter().enumerate() {
            puzzle_array[i][j] = item;
        }
    }

    puzzle_array
}