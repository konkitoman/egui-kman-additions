use eframe::{App, NativeOptions};
use egui_kman_additions::table::{Column, Table};

pub struct Context {
    buff: String,
}

impl Context {
    pub fn new() -> Self {
        Self {
            buff: String::from("Un focused!"),
        }
    }
}

impl App for Context {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let res = Column::new("res", |ui, value: &(i32, i32)| {
            ui.label(&(value.0 + value.1).to_string());
        })
        .min_width(20.0);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_singleline(&mut self.buff);
            Table::new()
                .set_values(vec![(32, 3), (20, 1), (124, 3), (3, 6), (321, 64), (33, 9)])
                .column(Column::new("x", |ui, value| {
                    ui.label(&value.0.to_string());
                }))
                .column(Column::new("y", |ui, value| {
                    ui.label(&value.1.to_string());
                }))
                .column(res)
                .show(ui);
        });
    }
}

pub fn main() {
    eframe::run_native(
        "table example",
        NativeOptions {
            ..Default::default()
        },
        Box::new(|_| Box::new(Context::new())),
    );
}
