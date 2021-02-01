use super::Label;
use super::*;
use bracket_geometry::prelude::Rect;

pub struct RetainedGui {
    bounds: Rect,
    elements: Vec<Box<dyn RetainedElement>>,
    focus_list: Vec<usize>,
    focused_id: Option<usize>,
    focused_index: usize,
}

impl RetainedGui {
    pub fn new(bounds: Rect) -> Self {
        Self {
            bounds,
            elements: Vec::new(),
            focus_list: Vec::new(),
            focused_id: None,
            focused_index: 0
        }
    }

    pub fn build(&mut self) {
        self.focus_list = self
            .elements
            .iter()
            .filter(|e| e.can_focus())
            .map(|e| e.get_id())
            .collect();

        if !self.focus_list.is_empty() {
            self.focused_id = Some(self.focus_list[0]);
            self.focused_index = 0;
        }
    }

    pub fn resize(&mut self, new_bounds: Rect) {
        self.bounds = new_bounds;
    }

    pub fn tick(&mut self, ctx: &mut BTerm) -> Option<RetainedGuiEvent> {
        let mut result = None;
        let bounds = self.bounds.clone();

        let fid = self.focused_id;
        self.elements.iter_mut().for_each(|e| {
            let has_focus = if let Some(id) = fid {
                e.get_id() == id
            } else {
                false
            };

            if let Some(event) = e.render(ctx, &bounds, has_focus) {
                result = Some(event)
            }
        });

        match ctx.key {
            Some(VirtualKeyCode::Tab) => {
                if !self.focus_list.is_empty() {
                    self.focused_index += 1;
                    if self.focused_index >= self.focus_list.len() {
                        self.focused_index = 0;
                    }
                    self.focused_id = Some(self.focus_list[self.focused_index]);
                }
            }
            _ => {}
        }

        result
    }

    pub fn add_label(&mut self, fg: RGB, bg: RGB, text: &str, x: i32, y: i32, id: usize) {
        self.elements.push(Box::new(Label {
            fg,
            bg,
            x,
            y,
            text: text.to_string(),
            id,
        }));
    }

    pub fn add_label_centered(&mut self, fg: RGB, bg: RGB, text: &str, y: i32, id: usize) {
        self.elements.push(Box::new(LabelCentered {
            fg,
            bg,
            y,
            text: text.to_string(),
            id,
        }));
    }

    pub fn add_double_box(&mut self, fg: RGB, bg: RGB, id: usize) {
        self.elements.push(Box::new(DoubleBox { fg, bg, id }));
    }

    pub fn add_button(&mut self, label: &str, x: i32, y: i32, fg: RGB, bg: RGB, id: usize) {
        self.elements.push(Box::new(Button {
            label: label.to_string(),
            fg,
            bg,
            x,
            y,
            id,
        }));
    }

    pub fn add_checkbox(&mut self, x: i32, y: i32, checked: bool, id: usize) {
        self.elements.push(Box::new(Checkbox { x, y, checked, id }));
    }

    pub fn add_integer_selector(&mut self, x: i32, y: i32, val: i32, min: i32, max: i32, id: usize) {
        self.elements.push(Box::new(IntegerInput {
            x,
            y,
            id,
            val,
            min,
            max
        }));
    }
}
