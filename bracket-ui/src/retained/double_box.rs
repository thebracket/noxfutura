use super::{RetainedElement, RetainedGuiEvent};
use bracket_terminal::prelude::*;

pub(crate) struct DoubleBox {
    pub(crate) bg: RGB,
    pub(crate) fg: RGB,
    pub(crate) id: usize,
}

impl RetainedElement for DoubleBox {
    fn render(
        &mut self,
        ctx: &mut BTerm,
        parent_bounds: &Rect,
        _has_focus: bool,
    ) -> Option<RetainedGuiEvent> {
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

    fn can_focus(&self) -> bool {
        false
    }

    fn get_id(&self) -> usize {
        self.id
    }
}
