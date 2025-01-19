mod solver;
mod generate;
mod utils;
mod sudoku;
mod difficulty;

use eframe::egui;
use crate::sudoku::SudokuApp;

fn main() -> eframe::Result {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([550.0, 700.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Sudoku",
        options,
        Box::new(|_cc| Ok(Box::<SudokuApp>::default())),
    )
}
