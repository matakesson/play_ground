use std::{cell, io::empty, time::{Duration, Instant}};

use eframe::egui;
use crate::sudoku::{Generator, Grid, Solver, generator::Difficulty};

pub struct SudokuApp{
    grid: Grid,
    selected_cell: Option<(usize, usize)>,
    game_won: bool,
    solving: bool,
    solver_state: Option<SolverState>,
    last_step_time: Instant,
    input_mode: bool,
}

struct SolverState {
    current_grid: Grid,
    original_fixed: [[bool; 9]; 9],
    stack: Vec<SolverStep>,
    speed_ms: u64,
}

#[derive(Clone)]
struct SolverStep {
    row: usize,
    col: usize,
    tried_numbers: Vec<u8>,
}

impl Default for SudokuApp {
    fn default() -> Self {
        Self {
            grid: Generator::generate(Difficulty::Medium),
            selected_cell: None,
            game_won: false,
            solving: false,
            solver_state: None,
            last_step_time: Instant::now(),
            input_mode: false,
        }
    }
}

impl eframe::App for SudokuApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.solving{
            ctx.request_repaint();
        }

        egui::CentralPanel::default().show(ctx, |ui|{
            ui.heading("Sudoku Game");
            ui.add_space(10.0);

            // Mode toggle button
            ui.horizontal(|ui| {
                ui.add_enabled_ui(!self.solving, |ui| {
                    if self.input_mode {
                        if ui.button("✓ Done - Lock Puzzle").clicked() {
                            // Lock all non-zero cells as fixed
                            for row in 0..9 {
                                for col in 0..9 {
                                    if self.grid.get(row, col) != 0 {
                                        self.grid.set_fixed(row, col, true);
                                    } else {
                                        self.grid.set_fixed(row, col, false);
                                    }
                                }
                            }
                            self.input_mode = false;
                            self.game_won = false;
                            self.selected_cell = None;
                        }
                        ui.colored_label(egui::Color32::from_rgb(255, 100, 100), "INPUT MODE: Enter your puzzle");
                    } else {
                        if ui.button("📝 Input Custom Puzzle").clicked() {
                            self.grid = Grid::new();
                            self.input_mode = true;
                            self.game_won = false;
                            self.selected_cell = None;
                        }
                    }
                });
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                ui.add_enabled_ui(!self.solving && !self.input_mode, |ui| {
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
            });

            ui.add_space(5.0);

            ui.horizontal(|ui| {
                if !self.solving && !self.input_mode {
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
                    if ui.button("Solve (Animated)").clicked(){
                        self.start_animated_solve();
                    }

                    if ui.button("Clear My Entries").clicked(){
                        self.grid.clear_non_fixed();
                        self.game_won = false;
                    }
                } else if self.input_mode {
                    if ui.button("Clear All").clicked() {
                        self.grid = Grid::new();
                        self.selected_cell = None;
                    }
                } else {
                    if ui.button("Stop Animation").clicked() {
                        self.solving = false;
                        self.solver_state = None;
                    }
                    ui.label("Solving...");
                }
            });

            if self.solving {
                ui.horizontal(|ui|{
                    ui.label("Speed:");
                    if let Some(ref mut state) = self.solver_state {
                        ui.add(egui::Slider::new(&mut state.speed_ms, 1..=500)
                            .text("ms"));
                    }
                });
            }

            ui.add_space(20.0);

            self.draw_grid(ui);

            if self.solving {
                let now = Instant::now();
                if let Some(ref state) = self.solver_state{
                    if now.duration_since(self.last_step_time) >= Duration::from_millis(state.speed_ms){
                        self.last_step_time = now;
                        self.step_solve();
                    }
                }
            }

            ui.add_space(20.0);
            if !self.solving {
                self.draw_number_buttons(ui);
            }
            
            if !self.input_mode && (self.game_won || self.grid.is_complete()){
                ui.add_space(10.0);
                ui.colored_label(egui::Color32::GREEN, "🎉 Congratulations! Puzzle solved! 🎉");
                self.game_won = true;
            }
        });
    }
}

impl SudokuApp {
    fn start_animated_solve(&mut self){
        let mut solving_grid = self.grid.clone();
        let mut original_fixed = [[false; 9]; 9];
        for row in 0..9{
            for col in 0..9 {
                original_fixed[row][col] = solving_grid.is_fixed(row, col);
            }
        }

        for row in 0..9{
            for col in 0..9 {
                solving_grid.set_fixed(row, col, false);
            }
        }

        self.solving = true;
        self.solver_state = Some(SolverState { 
            current_grid: solving_grid,
            original_fixed,
            stack: Vec::new(),
            speed_ms: 50
        });
        self.last_step_time = Instant::now()
    }

    fn step_solve(&mut self){
        if let Some(mut state) = self.solver_state.take() {
            let empty_pos = Solver::find_next_empty(&state.current_grid, 0, 0);

            if empty_pos.is_none(){
                self.grid = state.current_grid.clone();
                self.solving = false;
                self.game_won = true;
                return;
            }

            let (row, col) = empty_pos.unwrap();
            let mut found = false;
            for num in 1..=9 {
                if let Some(last_step) = state.stack.last() {
                    if last_step.row == row && last_step.col == col && last_step.tried_numbers.contains(&num){
                        continue;
                    }
                }

                if Solver::is_safe_static(&state.current_grid, row, col, num){
                    state.current_grid.set(row, col, num);
                    self.grid = state.current_grid.clone();
                    state.stack.push(SolverStep { row, col, tried_numbers: vec![num] });
                    found = true;
                    break;
                }
            }

            if !found {
                self.backtrack(&mut state);
            }
            self.solver_state = Some(state);
        }
    }


    fn draw_grid(&mut self, ui: &mut egui::Ui){
        let cell_size = 50.0;
        let grid_size = cell_size * 9.0;

        let (response, painter) = ui.allocate_painter(
            egui::Vec2::new(grid_size, grid_size), 
            egui::Sense::click(),
        );

        let origin = response.rect.min;

        if !self.solving && response.clicked() {  
            if let Some(pos) = response.interact_pointer_pos() {
                let relative = pos - origin;
                let col = (relative.x / cell_size) as usize;
                let row = (relative.y / cell_size) as usize;
                if row < 9 && col < 9 {
                    // In input mode, allow selecting any cell
                    // In play mode, only allow selecting non-fixed cells
                    if self.input_mode || !self.grid.is_fixed(row, col) {
                        self.selected_cell = Some((row, col));
                    }
                }
            }
        }

        let solving_cell = if let Some(ref state) = self.solver_state {
            if let Some(last_step) = state.stack.last() {
                Some((last_step.row, last_step.col))
            } else {
                None
            }
        } else {
            None
        };

        for row in 0..9 {
            for col in 0..9 {
                let rect = egui::Rect::from_min_size(
                    origin + egui::Vec2::new(col as f32 * cell_size, row as f32 * cell_size),
                    egui::Vec2::splat(cell_size),
                );

                let is_originally_fixed = if let Some(ref state) = self.solver_state {
                    state.original_fixed[row][col]
                } else {
                    self.grid.is_fixed(row, col)
                };

                let color = if Some((row, col)) == solving_cell {
                    egui::Color32::from_rgb(255, 200, 200) 
                } else if Some((row, col)) == self.selected_cell {
                    egui::Color32::from_rgb(200, 220, 255)
                } else if self.grid.is_fixed(row, col){
                    egui::Color32::from_rgb(240, 240, 240)
                } else {
                    egui::Color32::WHITE
                };
                painter.rect_filled(rect, 0.0, color);

                let value = self.grid.get(row, col);
                if value != 0 {
                    let text_color = if is_originally_fixed{
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

    fn backtrack(&mut self, state: &mut SolverState) {
        if let Some(mut last_step) = state.stack.pop() {
            let row = last_step.row;
            let col = last_step.col;

            state.current_grid.set(row, col, 0);
            self.grid = state.current_grid.clone();
            let last_num = *last_step.tried_numbers.last().unwrap();
            let mut found = false;
            for num in (last_num + 1)..=9 {
                if Solver::is_safe_static(&state.current_grid, row, col, num){
                    state.current_grid.set(row, col, num);
                    self.grid = state.current_grid.clone();
                    last_step.tried_numbers.push(num);
                    state.stack.push(last_step);
                    found = true;
                    break;
                }
            }
            if !found {
                self.backtrack(state);
            }
        }else {
            self.solving = false;
            self.solver_state = None;
        }
    }

    fn draw_number_buttons(&mut self, ui: &mut egui::Ui){
        if self.selected_cell.is_some(){
            ui.label("Select a number");
            ui.horizontal(|ui| {
                for num in 1..=9{
                    if ui.button(num.to_string()).clicked(){
                        if let Some((row, col)) = self.selected_cell{
                            if self.input_mode {
                                // In input mode, directly set the value
                                self.grid.set(row, col, num);
                            } else {
                                // In play mode, use set_user
                                self.grid.set_user(row, col, num);
                            }
                        }
                    }
                }
                if ui.button("Clear").clicked(){
                    if let Some((row, col)) = self.selected_cell{
                        if self.input_mode {
                            self.grid.set(row, col, 0);
                        } else {
                            self.grid.set_user(row, col, 0);
                        }
                    }
                }
            });
        } else {
            ui.label("Select a cell to enter a number");
        }
    }
}