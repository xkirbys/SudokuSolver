use eframe::emath::{Rect, Vec2};
use eframe::epaint::{Color32, Stroke};
use crate::difficulty::Difficulty;
use crate::generate::generate_sudoku;
use crate::solver::solve_sudoku;
use crate::utils::{copy_to_clipboard, read_from_clipboard, key_pressed};

#[derive(Default)]
pub struct SudokuApp {
    grid: [[Option<u8>; 9]; 9],
    selected_cell: Option<(usize, usize)>,
    difficulty: Difficulty,
    progress: f32,
}

impl SudokuApp {
    fn cell_rect(&self, rect: Rect, cell_size: f32, row: usize, col: usize) -> Rect {
        Rect::from_min_size(
            egui::pos2(
                rect.left() + (col as f32 * cell_size),
                rect.top() + (row as f32 * cell_size),
            ),
            Vec2::splat(cell_size),
        )
    }
}

impl eframe::App for SudokuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Sudoku");
            });
            ui.add_space(20.0);

            let available_width = ui.available_width().min(540.0);
            let cell_size = available_width / 9.0;

            // Create a square area for the grid
            let (response, painter) = ui.allocate_painter(
                Vec2::new(available_width, available_width),
                egui::Sense::click(),
            );
            let rect = response.rect;

            // Handle mouse input
            if let Some(pos) = response.hover_pos() {
                let x = ((pos.x - rect.left()) / cell_size) as usize;
                let y = ((pos.y - rect.top()) / cell_size) as usize;
                if x < 9 && y < 9 && response.clicked() {
                    self.selected_cell = Some((x, y));
                }
            }

            // Draw the background for selected cell
            if let Some((x, y)) = self.selected_cell {
                let cell_rect = self.cell_rect(rect, cell_size, y, x);
                painter.rect_filled(cell_rect, 0.0, Color32::from_gray(100));
            }

            // Draw vertical and horizontal grid lines
            for i in 0..=9 {
                let stroke = if i % 3 == 0 {
                    Stroke::new(2.0, Color32::WHITE)
                } else {
                    Stroke::new(0.5, Color32::from_gray(180))
                };

                let x = rect.left() + (i as f32 * cell_size);
                painter.line_segment(
                    [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                    stroke,
                );

                let y = rect.top() + (i as f32 * cell_size);
                painter.line_segment(
                    [egui::pos2(rect.left(), y), egui::pos2(rect.right(), y)],
                    stroke,
                );
            }

            // Draw numbers
            for row in 0..9 {
                for col in 0..9 {
                    if let Some(num) = self.grid[row][col] {
                        let cell_rect = self.cell_rect(rect, cell_size, row, col);

                        painter.text(
                            cell_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            num.to_string(),
                            egui::FontId::proportional(24.0),
                            Color32::WHITE,
                        );
                    }
                }
            }

            // Handle keyboard input
            if let Some((x, y)) = self.selected_cell {
                if ctx.input(|i| !i.keys_down.is_empty()) {
                    ctx.input(|i| {
                        for key in &i.keys_down {
                            if let Ok(num) = key.symbol_or_name().parse::<u8>() {
                                if (1..=9).contains(&num) {
                                    self.grid[y][x] = Some(num);
                                }
                            }
                        }
                        if key_pressed(i, egui::Key::Backspace) || key_pressed(i, egui::Key::Delete) {
                            self.grid[y][x] = None;
                        }
                    });
                }
            }

            // Add a progress bar text beneath the grid
            ui.add_space(20.0);
            ui.label("Progress:");
            ui.add(
                egui::ProgressBar::new(self.progress).text(format!("{:.2}%", self.progress * 100.0)),
            );


            ui.add_space(20.0);
            ui.horizontal_centered(|ui| {
                if ui.button("Clear").clicked() {
                    self.grid = Default::default();
                    self.selected_cell = None;
                }
                if ui.button("Solve").clicked() {
                    self.progress = 0.0;
                    solve_sudoku(&mut self.grid, &mut self.progress);
                }

                if ui.button("Generate").clicked() {
                    self.grid = generate_sudoku(self.difficulty);
                }
                egui::ComboBox::from_label("")
                    .selected_text(match self.difficulty {
                        Difficulty::Easy => "easy",
                        Difficulty::Medium => "medium",
                        Difficulty::Hard => "hard",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.difficulty, Difficulty::Easy, "Easy");
                        ui.selectable_value(&mut self.difficulty, Difficulty::Medium, "Medium");
                        ui.selectable_value(&mut self.difficulty, Difficulty::Hard, "Hard");
                    });

                if ui.button("Export").clicked() {
                    let json = serde_json::to_string(&self.grid).expect("Failed to serialize grid");
                    copy_to_clipboard(json).expect("Failed to copy to clipboard");
                }
                if ui.button("Import").clicked() {
                    if let Ok(text) = read_from_clipboard() {
                        match serde_json::from_str::<[[Option<u8>; 9]; 9]>(&text) {
                            Ok(parsed_grid) => {
                                self.grid = parsed_grid;
                            }
                            Err(e) => {
                                println!("Failed to parse grid: {}", e);
                            }
                        }
                    }

                }

                if ui.button("Backtracking TEST").clicked() {
                    self.grid = [
                        [None, None, None, None, None, None, None, None, None],
                        [None, None, None, None, None, Some(3), None, Some(8), Some(5)],
                        [None, None, Some(1), None, Some(2), None, None, None, None],
                        [None, None, None, Some(5), None, Some(7), None, None, None],
                        [None, None, Some(4), None, None, None, Some(1), None, None],
                        [None, Some(9), None, None, None, None, None, None, None],
                        [Some(5), None, None, None, None, None, None, Some(7), Some(3)],
                        [None, None, Some(2), None, Some(1), None, None, None, None],
                        [None, None, None, None, Some(4), None, None, None, Some(9)]
                    ];
                }

                if let Some((x, y)) = self.selected_cell {
                    let cell_value = self.grid[y][x]
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "Empty".to_string());
                    ui.label(format!("Selected cell: ({}, {}) - Value: {}", y + 1, x + 1, cell_value));
                } else {
                    ui.label("Selected cell: None");
                }
            });


        });
    }
}