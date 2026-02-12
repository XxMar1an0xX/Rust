use egui::{self, RawInput, Ui};
fn main() {
    let mut ctx = egui::Context::default();
    loop {
        egui::Window::new("hola").show(&ctx, |ui| {
            ui.label("asjdhasakshda");
        });
    }
}
