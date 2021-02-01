use super::{RetainedElement, RetainedGuiEvent};
use bracket_terminal::prelude::*;

pub(crate) struct Button {
    pub(crate) label: String,
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) fg: RGB,
    pub(crate) bg: RGB,
    pub(crate) id: usize,
}

impl RetainedElement for Button {
    fn render(
        &mut self,
        ctx: &mut BTerm,
        parent_bounds: &Rect,
        has_focus: bool,
    ) -> Option<RetainedGuiEvent> {
        let mp = ctx.mouse_pos();
        let hovered = (mp.0 >= parent_bounds.x1 - 1 + self.x
            && mp.0 <= parent_bounds.x1 - 1 + self.x + self.label.len() as i32 + 1
            && mp.1 >= parent_bounds.y1 - 1 + self.y
            && mp.1 <= parent_bounds.y1 - 1 + self.y + 3)
            || has_focus;

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
            return Some(RetainedGuiEvent::Click(self.id));
        }

        if has_focus {
            if let Some(VirtualKeyCode::Space) = ctx.key {
                return Some(RetainedGuiEvent::Click(self.id));
            }
        }

        None
    }

    fn can_focus(&self) -> bool {
        true
    }

    fn get_id(&self) -> usize {
        self.id
    }
}
