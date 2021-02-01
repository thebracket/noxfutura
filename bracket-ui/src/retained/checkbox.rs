use super::{RetainedElement, RetainedGuiEvent};
use bracket_terminal::prelude::*;

pub(crate) struct Checkbox {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) checked: bool,
    pub(crate) id: usize,
}

impl RetainedElement for Checkbox {
    fn render(
        &mut self,
        ctx: &mut BTerm,
        parent_bounds: &Rect,
        has_focus: bool,
    ) -> Option<RetainedGuiEvent> {
        if has_focus {
            if self.checked {
                ctx.print_color(
                    parent_bounds.x1 + self.x,
                    parent_bounds.y1 + self.y,
                    GREEN,
                    BLACK,
                    "[ ]",
                );
            } else {
                ctx.print_color(
                    parent_bounds.x1 + self.x,
                    parent_bounds.y1 + self.y,
                    GREEN,
                    BLACK,
                    "[X]",
                );
            }
        } else {
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
        }

        let mp = ctx.mouse_pos();
        if mp.1 == parent_bounds.y1 + self.y
            && mp.0 >= parent_bounds.x1 + self.x
            && mp.0 <= parent_bounds.x1 + self.x + 3
            && ctx.left_click
        {
            self.checked = !self.checked;
            return Some(RetainedGuiEvent::Checkbox(self.id, self.checked));
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
