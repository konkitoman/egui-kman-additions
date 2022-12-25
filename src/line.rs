use egui::{Color32, Id, Rect, Response, Rounding, Stroke, Ui, Vec2, Widget};

pub struct Line {
    pub fill_vertical: bool,
    pub fill_horizontal: bool,
    // pub margins: Margin,
    pub width: f32,
    pub height: f32,
}

pub struct LinePrepare<'a> {
    pub response: Response,
    pub ui: &'a mut Ui,
    pub id: Id,
    pub rect: Rect,
    pub rounding: Rounding,
    pub stroke: Stroke,
    pub color: Color32,
}

impl Line {
    pub fn new() -> Self {
        Self {
            fill_vertical: true,
            fill_horizontal: false,
            // margins: Margin::same(0.0),
            width: 3.0,
            height: 3.0,
        }
    }

    pub fn fill(self, value: [bool; 2]) -> Self {
        let mut s = self;
        s.fill_vertical = value[0];
        s.fill_horizontal = value[1];
        s
    }

    pub fn vertical(self) -> Self {
        let mut s = self;
        s.fill_vertical = true;
        s.fill_horizontal = false;
        s
    }

    pub fn horizontal(self) -> Self {
        let mut s = self;
        s.fill_vertical = false;
        s.fill_horizontal = true;
        s
    }

    pub fn width(self, value: f32) -> Self {
        let mut s = self;
        s.width = value;
        s
    }

    pub fn height(self, value: f32) -> Self {
        let mut s = self;
        s.height = value;
        s
    }

    pub fn beagin(self, ui: &mut Ui) -> LinePrepare {
        let mut width = self.width;
        let mut height = self.height;

        if self.fill_vertical {
            width = ui.available_width();
        }

        if self.fill_horizontal {
            height = ui.available_height();
        }

        let (id, rect) = ui.allocate_space(Vec2::new(width, height));

        LinePrepare {
            response: ui.interact(rect, id, egui::Sense::click_and_drag()),
            rounding: ui.style().visuals.window_rounding,
            stroke: ui.style().visuals.window_stroke(),
            color: ui.style().visuals.faint_bg_color,
            ui,
            id,
            rect,
        }
    }
}

impl<'a> LinePrepare<'a> {
    pub fn response(&self) -> &Response {
        &self.response
    }
    pub fn id(&self) -> &Id {
        &self.id
    }
    pub fn rounding(self, value: Rounding) -> Self {
        let mut s = self;
        s.rounding = value;
        s
    }
    pub fn stroke(self, value: Stroke) -> Self {
        let mut s = self;
        s.stroke = value;
        s
    }
    pub fn color(self, value: Color32) -> Self {
        let mut s = self;
        s.color = value;
        s
    }
    pub fn end(self) -> Response {
        self.ui
            .painter()
            .rect(self.rect, self.rounding, self.color, self.stroke);
        self.response
    }
}

impl Widget for Line {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let beagin = self.beagin(ui);
        beagin.end()
    }
}
