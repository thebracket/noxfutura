use super::{RetainedElement, RetainedGuiEvent};
use bracket_terminal::prelude::*;

pub(crate) struct LabelCentered {
    pub(crate) text: String,
    pub(crate) bg: RGB,
    pub(crate) fg: RGB,
    pub(crate) y: i32,
    pub(crate) id: usize,
}

impl RetainedElement for LabelCentered {
    fn render(
        &mut self,
        ctx: &mut BTerm,
        parent_bounds: &Rect,
        _has_focus: bool,
    ) -> Option<RetainedGuiEvent> {
        ctx.print_color_centered_at(
            parent_bounds.center().x,
            self.y + parent_bounds.y1,
            self.fg,
            self.bg,
            &self.text,
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
