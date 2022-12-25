use eframe::{App, NativeOptions};

pub struct State;
use egui_kman_additions::SingleLineComplition;

impl App for State {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.add(SingleLineComplition::new(
                        "testing",
                        &[
                            "", "Test1", "Test2", "Test21", "qqq", "dd", "Test6", "Test 8",
                            "Test10", "Test432", "testdsa", "mqq", "dds",
                        ],
                    ));
                });
            });
        });
    }
}

pub fn main() {
    eframe::run_native(
        "single line complition",
        NativeOptions::default(),
        Box::new(|_| Box::new(State)),
    );
}
