use imgui::Ui;
use crate::Textures;

pub struct Core<'a> {
    pub imgui: &'a Ui::<'a>,
    pub textures: &'a mut Textures
}