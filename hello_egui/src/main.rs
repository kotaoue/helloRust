use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_resizable(false)
            .with_transparent(true),
        ..Default::default()
    };
    eframe::run_native(
        "Helloworld for egui",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

#[derive(Default)]
struct MyApp;

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let magenta = egui::Color32::from_rgb(255, 0, 255);

        ui.label(
            egui::RichText::new("Hello, World!")
                .heading()
                .color(egui::Color32::WHITE)
                .background_color(magenta),
        );
        ui.label(
            egui::RichText::new("This is a simple egui application.")
                .color(egui::Color32::WHITE)
                .background_color(magenta),
        );
    }

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        egui::Color32::from_rgba_unmultiplied(255, 255, 255, 160).to_normalized_gamma_f32()
    }
}
