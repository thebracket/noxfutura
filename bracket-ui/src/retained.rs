use bracket_geometry::prelude::Rect;
use bracket_terminal::prelude::*;

pub enum RetainedGuiEvent {
    Click(String),
    Checkbox(bool, usize),
}

pub struct RetainedGui {
    bounds: Rect,
    elements: Vec<Box<dyn RetainedElement>>,
}

impl RetainedGui {
    pub fn new(bounds: Rect) -> Self {
        Self {
            bounds,
            elements: Vec::new(),
        }
    }

    pub fn resize(&mut self, new_bounds: Rect) {
        self.bounds = new_bounds;
    }

    pub fn tick(&mut self, ctx: &mut BTerm) -> Option<RetainedGuiEvent> {
        let mut result = None;
        let bounds = self.bounds.clone();

        self.elements.iter_mut().for_each(|e| {
            if let Some(event) = e.render(ctx, &bounds) {
                result = Some(event)
            }
        });

        result
    }

    pub fn add_label(&mut self, fg: RGB, bg: RGB, text: &str, x: i32, y: i32) {
        self.elements.push(Box::new(Label {
            fg,
            bg,
            x,
            y,
            text: text.to_string(),
        }));
    }

    pub fn add_label_centered(&mut self, fg: RGB, bg: RGB, text: &str, y: i32) {
        self.elements.push(Box::new(LabelCentered {
            fg,
            bg,
            y,
            text: text.to_string(),
        }));
    }

    pub fn add_double_box(&mut self, fg: RGB, bg: RGB) {
        self.elements.push(Box::new(DoubleBox { fg, bg }));
    }

    pub fn add_button(&mut self, label: &str, x: i32, y: i32, fg: RGB, bg: RGB) {
        self.elements.push(Box::new(Button {
            label: label.to_string(),
            fg,
            bg,
            x,
            y,
        }));
    }

    pub fn add_checkbox(&mut self, x: i32, y: i32, checked: bool, id: usize) {
        self.elements.push(Box::new(Checkbox { x, y, checked, id }));
    }
}

trait RetainedElement {
    fn render(&mut self, ctx: &mut BTerm, parent_bounds: &Rect) -> Option<RetainedGuiEvent>;
}

struct Label {
    text: String,
    bg: RGB,
    fg: RGB,
    x: i32,
    y: i32,
}

impl RetainedElement for Label {
    fn render(&mut self, ctx: &mut BTerm, parent_bounds: &Rect) -> Option<RetainedGuiEvent> {
        ctx.print_color(
            self.x + parent_bounds.x1,
            self.y + parent_bounds.y1,
            self.fg,
            self.bg,
            &self.text,
        );
        None
    }
}

struct LabelCentered {
    text: String,
    bg: RGB,
    fg: RGB,
    y: i32,
}

impl RetainedElement for LabelCentered {
    fn render(&mut self, ctx: &mut BTerm, parent_bounds: &Rect) -> Option<RetainedGuiEvent> {
        ctx.print_color_centered_at(
            parent_bounds.center().x,
            self.y + parent_bounds.y1,
            self.fg,
            self.bg,
            &self.text,
        );
        None
    }
}

struct DoubleBox {
    bg: RGB,
    fg: RGB,
}

impl RetainedElement for DoubleBox {
    fn render(&mut self, ctx: &mut BTerm, parent_bounds: &Rect) -> Option<RetainedGuiEvent> {
        ctx.draw_box_double(
            parent_bounds.x1,
            parent_bounds.y1,
            parent_bounds.width(),
            parent_bounds.height(),
            self.fg,
            self.bg,
        );
        None
    }
}

struct Button {
    label: String,
    x: i32,
    y: i32,
    fg: RGB,
    bg: RGB,
}

impl RetainedElement for Button {
    fn render(&mut self, ctx: &mut BTerm, parent_bounds: &Rect) -> Option<RetainedGuiEvent> {
        let mp = ctx.mouse_pos();
        let hovered = mp.0 >= parent_bounds.x1 - 1 + self.x
            && mp.0 <= parent_bounds.x1 - 1 + self.x + self.label.len() as i32 + 1
            && mp.1 >= parent_bounds.y1 - 1 + self.y
            && mp.1 <= parent_bounds.y1 - 1 + self.y + 3;

        if !hovered {
            ctx.draw_box(
                parent_bounds.x1 - 1 + self.x,
                parent_bounds.y1 - 1 + self.y,
                self.label.len() + 1,
                2,
                self.fg,
                self.bg,
            );
            ctx.print_color(
                parent_bounds.x1 + self.x,
                parent_bounds.y1 + self.y,
                self.fg,
                self.bg,
                &self.label,
            );
        } else {
            ctx.draw_box(
                parent_bounds.x1 - 1 + self.x,
                parent_bounds.y1 - 1 + self.y,
                self.label.len() + 1,
                2,
                BLACK,
                YELLOW,
            );
            ctx.print_color(
                parent_bounds.x1 + self.x,
                parent_bounds.y1 + self.y,
                BLACK,
                YELLOW,
                &self.label,
            );
        }

        if ctx.left_click && hovered {
            return Some(RetainedGuiEvent::Click(self.label.clone()));
        }

        None
    }
}

struct Checkbox {
    x: i32,
    y: i32,
    checked: bool,
    id: usize,
}

impl RetainedElement for Checkbox {
    fn render(&mut self, ctx: &mut BTerm, parent_bounds: &Rect) -> Option<RetainedGuiEvent> {
        if self.checked {
            ctx.print_color(
                parent_bounds.x1 + self.x,
                parent_bounds.y1 + self.y,
                GRAY,
                BLACK,
                "[ ]",
            );
        } else {
            ctx.print_color(
                parent_bounds.x1 + self.x,
                parent_bounds.y1 + self.y,
                WHITE,
                BLACK,
                "[X]",
            );
        }

        let mp = ctx.mouse_pos();
        if mp.1 == parent_bounds.y1 + self.y
            && mp.0 >= parent_bounds.x1 + self.x
            && mp.0 <= parent_bounds.x1 + self.x + 3
            && ctx.left_click
        {
            self.checked = !self.checked;
            return Some(RetainedGuiEvent::Checkbox(self.checked, self.id));
        }

        None
    }
}
