use super::{RetainedElement, RetainedGuiEvent};
use bracket_terminal::prelude::*;

pub(crate) struct IntegerInput {
    pub(crate) min: i32,
    pub(crate) max: i32,
    pub(crate) val: i32,
    pub(crate) id: usize,
    pub(crate) x: i32,
    pub(crate) y: i32,
}

impl RetainedElement for IntegerInput {
    fn render(
        &mut self,
        ctx: &mut BTerm,
        parent_bounds: &Rect,
        has_focus: bool,
    ) -> Option<RetainedGuiEvent> {
        let fg = if has_focus {
            RGB::named(WHITE)
        } else {
            RGB::named(WHITE)
        };

        let bg = if has_focus {
            RGB::named(DARK_GREEN)
        } else {
            RGB::named(BLACK)
        };

        ctx.print_color(
            parent_bounds.x1 + self.x,
            parent_bounds.y1 + self.y,
            fg,
            bg,
            &format!("[{}] +/-", self.val)
        );

        if has_focus {
            match ctx.key {
                Some(VirtualKeyCode::Up) => {
                    self.val += 1;
                    self.val = i32::min(self.val, self.max);
                    return Some(RetainedGuiEvent::IntegerChange(self.id, self.val));
                }
                Some(VirtualKeyCode::Down) => {
                    self.val -= 1;
                    self.val = i32::max(self.val, self.min);
                    return Some(RetainedGuiEvent::IntegerChange(self.id, self.val));
                }
                _ => {}
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
