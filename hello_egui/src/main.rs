use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Hello egui",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Hello, World!");
    }
}
