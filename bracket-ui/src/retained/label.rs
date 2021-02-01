use super::{RetainedElement, RetainedGuiEvent};
use bracket_terminal::prelude::*;

pub(crate) struct Label {
    pub(crate) text: String,
    pub(crate) bg: RGB,
    pub(crate) fg: RGB,
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) id: usize,
}

impl RetainedElement for Label {
    fn render(
        &mut self,
        ctx: &mut BTerm,
        parent_bounds: &Rect,
        _has_focus: bool,
    ) -> Option<RetainedGuiEvent> {
        ctx.print_color(
            self.x + parent_bounds.x1,
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
