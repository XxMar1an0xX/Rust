use eframe::{
    NativeOptions,
    egui::{self, CentralPanel},
};

#[derive(Default)]
struct Aplicacion {}
impl eframe::App for Aplicacion {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("asjdhssak");
        });
    }
    // add code here
}

fn main() -> Result<(), eframe::Error> {
    let opciones = NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_resizable(true)
            .with_inner_size([300.0, 350.0]),
        ..Default::default()
    };
    eframe::run_native(
        "asdsad",
        opciones,
        Box::new(|_cc| Ok(Box::<Aplicacion>::default())),
    )
}
