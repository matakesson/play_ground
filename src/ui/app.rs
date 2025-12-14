use eframe::egui;
use crate::sudoku::{Generator, Grid, Solver, generator::Difficulty};

pub struct SudokuApp{
    grid: Grid,
    selected_cell: Option<(usize, usize)>,
    game_won: bool,
}

impl Default for SudokuApp {
    fn default() -> Self {
        Self {
            grid: Generator::generate(Difficulty::Medium),
            selected_cell: None,
            game_won: false,
        }
    }
}

impl eframe::App for SudokuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.heading("Sudoku Game");
            ui.add_space(10.0);

            ui.horizontal(|ui| {
                if ui.button("New Game (Easy)").clicked() {
                    self.grid = Generator::generate(Difficulty::Easy);
                    self.selected_cell = None;
                    self.game_won = false;
                }
                if ui.button("New Game (Medium)").clicked() {
                    self.grid = Generator::generate(Difficulty::Medium);
                    self.selected_cell = None;
                    self.game_won = false;
                }
                if ui.button("New Game (Hard)").clicked(){
                    self.grid = Generator::generate(Difficulty::Hard);
                    self.selected_cell = None;
                    self.game_won = false;
                }
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                if ui.button("Solve").clicked() {
                    let mut solving_grid = self.grid.clone();

                    for row in 0..9 {
                        for col in 0..9 {
                            solving_grid.set_fixed(row, col, false);
                        }
                    }

                    if Solver::solve(&mut solving_grid){
                        for row in 0..9{
                            for col in 0..9{
                                let value = solving_grid.get(row, col);
                                self.grid.set(row, col, value);
                            }
                        }
                        self.game_won = true;
                    }
                }
                if ui.button("Clear My Entries").clicked(){
                    self.grid.clear_non_fixed();
                    self.game_won = false;
                }
            });

            ui.add_space(20.0);

            self.draw_grid(ui);

            ui.add_space(20.0);
            self.draw_number_buttons(ui);

            if self.game_won || self.grid.is_complete(){
                ui.add_space(10.0);
                ui.colored_label(egui::Color32::GREEN, "🎉 Congratulations! Puzzle solved! 🎉");
                self.game_won = true;
            }
        });
    }
}

impl SudokuApp {
    fn draw_grid(&mut self, ui: &mut egui::Ui){
        let cell_size = 50.0;
        let grid_size = cell_size * 9.0;

        let (response, painter) = ui.allocate_painter(
            egui::Vec2::new(grid_size, grid_size), 
            egui::Sense::click(),
        );

        let origin = response.rect.min;

        if response.clicked() {
            if let Some(pos) = response.interact_pointer_pos() {
                let relative = pos - origin;
                let col = (relative.x / cell_size) as usize;
                let row = (relative.y / cell_size) as usize;
                if row < 9 && col < 9 && !self.grid.is_fixed(row, col) {
                    self.selected_cell = Some((row, col));
                }
            }
        }

        for row in 0..9 {
            for col in 0..9 {
                let rect = egui::Rect::from_min_size(
                    origin + egui::Vec2::new(col as f32 * cell_size, row as f32 * cell_size),
                    egui::Vec2::splat(cell_size),
                );

                let color = if Some((row, col)) == self.selected_cell {
                    egui::Color32::from_rgb(200, 220, 255)
                } else if self.grid.is_fixed(row, col){
                    egui::Color32::from_rgb(240, 240, 240)
                } else {
                    egui::Color32::WHITE
                };
                painter.rect_filled(rect, 0.0, color);

                let value = self.grid.get(row, col);
                if value != 0 {
                    let text_color = if self.grid.is_fixed(row, col){
                        egui::Color32::BLACK
                    } else {
                        egui::Color32::BLUE
                    };
                    painter.text(rect.center(), 
                    egui::Align2::CENTER_CENTER, 
                    value.to_string(), 
                    egui::FontId::proportional(30.0), 
                    text_color,
                    );
                }

                painter.rect_stroke(rect, 0.0, egui::Stroke::new(1.0, egui::Color32::GRAY));
            }
        }

        for i in 0..=3{
            let offset = i as f32 * cell_size * 3.0;
            painter.line_segment(
                [origin + egui::Vec2::new(0.0, offset),
                origin + egui::Vec2::new(grid_size, offset)], 
                egui::Stroke::new(3.0, egui::Color32::BLACK),
            );
            painter.line_segment(
                [origin + egui::Vec2::new(offset, 0.0),
                origin + egui::Vec2::new(offset, grid_size)], 
                egui::Stroke::new(3.0, egui::Color32::BLACK),
                );
        }
    }

    fn draw_number_buttons(&mut self, ui: &mut egui::Ui){
        if self.selected_cell.is_some(){
            ui.label("Select a number");
            ui.horizontal(|ui| {
                for num in 1..=9{
                    if ui.button(num.to_string()).clicked(){
                        if let Some((row, col)) = self.selected_cell{
                            self.grid.set_user(row, col, num);
                        }
                    }
                }
                if ui.button("Clear").clicked(){
                    if let Some((row, col)) = self.selected_cell{
                        self.grid.set_user(row, col, 0);
                    }
                }
            });
        } else {
            ui.label("Select a cell to enter a number");
        }
    }
}