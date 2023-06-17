use egui::{Context, Id};

#[derive(Clone)]
pub struct SingleLineComplitionState {
    buffer: String,
    finded: bool,
    finded_exact: bool,
    complitions: Vec<String>,
}

impl Default for SingleLineComplitionState {
    fn default() -> Self {
        Self {
            buffer: String::from(""),
            finded: false,
            finded_exact: false,
            complitions: Vec::new(),
        }
    }
}

pub struct SingleLineComplition<'a> {
    default: String,
    complitions: &'a [&'a str],
    exact: bool,
}

impl<'a> egui::Widget for SingleLineComplition<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        self.show(ui).response
    }
}

pub struct SingleLineComplitionOut {
    pub response: egui::Response,
    pub value: Option<String>,
    pub value_selected: bool,
}

impl<'a> SingleLineComplition<'a> {
    pub fn new(default: impl Into<String>, complitions: &'a [&str]) -> Self {
        Self {
            default: default.into(),
            complitions,
            exact: false,
        }
    }

    pub fn show(self, ui: &mut egui::Ui) -> SingleLineComplitionOut {
        let id = ui.id();
        let mut state =
            SingleLineComplitionState::load(ui.ctx(), id).unwrap_or(SingleLineComplitionState {
                buffer: self.default.clone(),
                finded: false,
                finded_exact: false,
                complitions: Vec::new(),
            });

        let mut value_selected = false;

        let res = ui
            .with_layout(egui::Layout::left_to_right(egui::Align::default()), |ui| {
                let res = ui.text_edit_singleline(&mut state.buffer);
                if res.lost_focus() {
                    state.complitions = Vec::new();
                    state.finded = false;
                    state.finded_exact = false;
                    for complition in self.complitions {
                        if state.buffer.is_empty() {
                            state.finded = true;
                            state.complitions.push(complition.to_string())
                        } else {
                            if let Some(_) = complition.trim().find(state.buffer.trim()) {
                                state.finded = true;
                                state.complitions.push(complition.to_string())
                            }
                        }

                        if complition.trim() == state.buffer.trim() {
                            state.finded_exact = true;
                        }
                    }
                }

                ui.memory_mut(|memory| memory.open_popup(id));
                if state.finded && !state.finded_exact {
                    egui::popup::popup_below_widget(ui, id, &res, |ui| {
                        let text_size = ui.text_style_height(&egui::TextStyle::Body);
                        egui::ScrollArea::new([true, true]).show_rows(
                            ui,
                            text_size,
                            state.complitions.len(),
                            |ui, range| {
                                let mut pressed = false;
                                for i in range {
                                    let complition = &state.complitions[i];
                                    if ui.small_button(complition).clicked() {
                                        state.buffer = complition.clone();
                                        state.finded_exact = true;
                                        pressed = true;
                                    }
                                }
                                if pressed {
                                    state.complitions.clear();
                                    value_selected = true;
                                }
                            },
                        );
                    });
                }
                let mut finded = state.finded_exact;
                ui.checkbox(&mut finded, "");
                res
            })
            .inner;

        let mut value = None;
        if self.exact {
            if state.finded_exact {
                value = Some(state.buffer.clone())
            }
        } else {
            value = Some(state.buffer.clone())
        }

        state.store(ui.ctx(), id);

        SingleLineComplitionOut {
            response: res,
            value_selected,
            value,
        }
    }

    pub fn exact(self, value: bool) -> Self {
        let mut r = self;
        r.exact = value;
        r
    }
}

impl SingleLineComplitionState {
    pub fn load(ctx: &Context, id: Id) -> Option<Self> {
        ctx.data_mut(|state| state.get_persisted(id))
    }

    pub fn store(self, ctx: &Context, id: Id) {
        ctx.data_mut(|state| state.insert_persisted(id, self))
    }
}
