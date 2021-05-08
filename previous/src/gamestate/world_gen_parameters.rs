use super::{stacked_state::StateInstruction, StackedState};
use crate::bracket_ui::color_menu::{ColorMenu, ColorMenuItem};
use crate::bracket_ui::retained::{LabelCenter, RetainedGui, SingleBox, XpBackground};
use bracket_lib::prelude::*;

pub struct WorldGenParameters {
    ui: RetainedGui,
}

impl WorldGenParameters {
    pub fn new() -> Box<Self> {
        let mut ui = RetainedGui::new();
        ui.add_element(XpBackground::new(
            "bg",
            Rect::with_size(0, 0, 110, 80),
            "../resources/background_image_ascii.rex",
        ));

        Box::new(Self { ui })
    }
}

impl StackedState for WorldGenParameters {
    fn run(&mut self, ctx: &mut BTerm) -> StateInstruction {
        self.ui.render(ctx);
        StateInstruction::CONTINUE
    }
}
