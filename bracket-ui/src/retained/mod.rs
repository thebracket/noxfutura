mod events;
pub use events::*;
mod gui;
pub use gui::*;
mod label;
pub(crate) use label::Label;
mod label_center;
pub(crate) use label_center::LabelCentered;
mod double_box;
pub(crate) use double_box::DoubleBox;
mod button;
pub(crate) use button::Button;
mod checkbox;
pub(crate) use checkbox::Checkbox;

use bracket_geometry::prelude::Rect;
use bracket_terminal::prelude::*;

trait RetainedElement {
    fn render(
        &mut self,
        ctx: &mut BTerm,
        parent_bounds: &Rect,
        has_focus: bool,
    ) -> Option<RetainedGuiEvent>;
    fn can_focus(&self) -> bool;
    fn get_id(&self) -> usize;
}
