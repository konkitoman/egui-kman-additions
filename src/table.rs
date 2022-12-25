use egui::{Color32, Id, Key, Sense, Ui, Vec2, Widget};

use crate::Line;

pub struct Column<T> {
    pub title: String,
    pub min_width: f32,
    pub width: f32,
    pub block: fn(&mut Ui, &T),
    pub title_centered: bool,
    pub centered: bool,
}

impl<T> Column<T> {
    pub fn new(title: &str, block: fn(&mut Ui, &T)) -> Self {
        Self {
            title: title.to_owned(),
            min_width: 20.0,
            width: 100.0,
            block,
            title_centered: false,
            centered: false,
        }
    }

    pub fn title_centered(self, value: bool) -> Self {
        let mut s = self;
        s.title_centered = value;
        s
    }

    pub fn contered(self, value: bool) -> Self {
        let mut s = self;
        s.centered = value;
        s
    }

    pub fn min_width(self, value: f32) -> Self {
        let mut s = self;
        s.min_width = value;
        s
    }

    pub fn width(self, value: f32) -> Self {
        let mut s = self;
        s.width = value;
        s
    }
}

pub struct Table<T> {
    id: Option<Id>,
    values: Vec<T>,
    columns: Vec<Column<T>>,
    invert_rows: bool,
    invert_columns: bool,
}

#[derive(Default, Clone)]
pub struct TableState {
    pub selected: Option<usize>,
    pub columns_size: Vec<f32>,
}

pub struct TableResponse {
    pub selected: Option<usize>,
    pub responses: egui::Response,
}

impl<T> Table<T> {
    pub fn new() -> Self {
        Self {
            id: None,
            values: Vec::new(),
            columns: Vec::new(),
            invert_rows: true,
            invert_columns: false,
        }
    }

    pub fn new_with(id: impl std::hash::Hash) -> Self {
        Self {
            id: Some(Id::new(id)),
            ..Self::new()
        }
    }

    pub fn set_values(self, values: Vec<T>) -> Self {
        let mut s = self;
        s.values = values;
        s
    }

    pub fn column(self, column: Column<T>) -> Self {
        let mut s = self;
        s.columns.push(column);
        s
    }

    pub fn show(self, ui: &mut Ui) -> TableResponse {
        let mut id = ui.id();

        if let Some(tmp_id) = self.id {
            id = tmp_id;
        }

        let mut columns = self.columns;
        if self.invert_columns {
            columns.reverse();
        }

        let mut values = self.values;
        let values_len = values.len();

        if self.invert_rows {
            values.reverse();
        }

        let mut state;
        {
            let s = ui.memory().data.get_temp::<TableState>(id);
            if let Some(s) = s {
                state = s
            } else {
                state = TableState::default()
            }
        }

        if state.columns_size.len() < columns.len() {
            state.columns_size.clear();
            for column in columns.iter() {
                state.columns_size.push(column.width);
            }
        }

        let mut focused = false;

        let res = ui.allocate_ui_with_layout(
            ui.available_size(),
            egui::Layout::top_down(egui::Align::Min),
            |ui| {
                egui::ScrollArea::horizontal()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        egui::Frame::group(&*ui.style())
                            .fill(Color32::from_rgb(25, 25, 25))
                            .show(ui, |ui| {
                                ui.allocate_space(Vec2::new(
                                    ui.available_size_before_wrap().x,
                                    0.0,
                                ));
                                ui.horizontal(|ui| {
                                    for (e, column) in columns.iter().enumerate() {
                                        ui.push_id(Id::new(&column.title), |ui| {
                                            let (_, rect) = ui.allocate_space(Vec2::new(
                                                state.columns_size[e],
                                                20.0,
                                            ));
                                            let ui = &mut ui.child_ui(
                                                rect,
                                                egui::Layout::left_to_right(egui::Align::Center)
                                                    .with_main_justify(column.title_centered),
                                            );
                                            egui::Label::new(
                                                egui::RichText::new(&column.title)
                                                    .color(Color32::from_rgb(255, 255, 255)),
                                            )
                                            .wrap(false)
                                            .ui(ui);
                                        });
                                        if e != columns.len() - 1 {
                                            let line =
                                                Line::new().horizontal().width(2.0).beagin(ui);
                                            state.columns_size[e] += line.response().drag_delta().x;
                                            if state.columns_size[e] < column.min_width {
                                                state.columns_size[e] = column.min_width;
                                            }
                                            line.end();
                                        }
                                    }
                                });
                            });

                        ui.separator();
                        egui::ScrollArea::vertical()
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                let mut uid = 0;

                                for i in 0..values.len() {
                                    let mut color = ui.style().visuals.faint_bg_color;
                                    if state.selected.is_some() && state.selected.unwrap() == i {
                                        color = ui.style().visuals.selection.bg_fill;
                                    }

                                    let value = values.pop().unwrap();
                                    let f = egui::Frame::group(&*ui.style()).fill(color).show(
                                        ui,
                                        |ui| {
                                            ui.allocate_space(Vec2::new(
                                                ui.available_size_before_wrap().x,
                                                0.0,
                                            ));
                                            ui.horizontal(|ui| {
                                                for (e, column) in columns.iter().enumerate() {
                                                    if e > 0 {
                                                        Line::new()
                                                            .horizontal()
                                                            .width(2.0)
                                                            .beagin(ui)
                                                            .end();
                                                        // ui.separator();
                                                    }
                                                    ui.push_id(
                                                        Id::new(&format!(
                                                            "{}+{}",
                                                            column.title, uid
                                                        )),
                                                        |ui| {
                                                            egui::ScrollArea::horizontal()
                                                                .min_scrolled_width(0.0)
                                                                .auto_shrink([false, false])
                                                                .min_scrolled_width(
                                                                    state.columns_size[e],
                                                                )
                                                                .max_width(state.columns_size[e])
                                                                .show(ui, |ui| {
                                                                    ui.allocate_ui_with_layout(
                                                            Vec2::new(column.min_width, 20.0),
                                                            egui::Layout::left_to_right(
                                                                egui::Align::Center,
                                                            )
                                                            .with_main_justify(column.centered),
                                                            |ui| {
                                                                (column.block)(ui, &value);
                                                            },
                                                        );
                                                                });
                                                        },
                                                    );
                                                    uid += 1;
                                                }
                                            });
                                        },
                                    );

                                    let r = ui.interact(
                                        f.response.rect,
                                        f.response.id,
                                        Sense::click_and_drag(),
                                    );

                                    if r.clicked() {
                                        focused = true;
                                        ui.memory().request_focus(id);
                                        if state.selected.is_some() && state.selected.unwrap() == i
                                        {
                                            state.selected = None;
                                        } else {
                                            state.selected = Some(i);
                                        }
                                    }
                                }
                            });
                    });
            },
        );

        let r = ui.interact_with_hovered(
            res.response.rect,
            false,
            id,
            Sense::focusable_noninteractive(),
        );

        if focused {
            r.request_focus();
        }

        if focused || r.has_focus() {
            if ui.input().key_pressed(Key::ArrowUp) {
                if let Some(selected) = &mut state.selected {
                    if *selected > 0 {
                        *selected -= 1;
                    } else {
                        *selected = values_len - 1;
                    }
                }
            }

            if ui.input().key_pressed(Key::ArrowDown) {
                if let Some(selected) = &mut state.selected {
                    if *selected < values_len - 1 {
                        *selected += 1;
                    } else {
                        *selected = 0;
                    }
                }
            }

            if ui.input().key_pressed(Key::Home) {
                if let Some(selected) = &mut state.selected {
                    *selected = 0;
                }
            }

            if ui.input().key_pressed(Key::End) {
                if let Some(selected) = &mut state.selected {
                    *selected = values_len - 1;
                }
            }
        }

        let selected = state.selected.clone();

        ui.memory().data.insert_temp(id, state);

        TableResponse {
            selected,
            responses: res.response,
        }
    }
}

impl<T> Widget for Table<T> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        self.show(ui).responses
    }
}
