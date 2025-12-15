use eframe::egui;
use sudoku_app::ui::app::SudokuApp;

fn main() -> Result<(), eframe::Error> {
    println!("Starting Sudoku app...");
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([600.0, 700.0])
            .with_resizable(true)
            .with_title("Sudoku"),
        ..Default::default()
    };

    println!("Creating window...");
    
    eframe::run_native(
        "Sudoku",
        options,
        Box::new(|_cc| {
            println!("App created!");
            Ok(Box::new(SudokuApp::default()))
        }),
    )
}