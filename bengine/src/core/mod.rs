mod init;
use imgui::Ui;
use crate::Textures;
pub use init::Initializer;

pub struct Core<'a> {
    pub imgui: &'a Ui::<'a>,
    pub textures: &'a mut Textures
}