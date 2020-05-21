use crate::opengl::*;

pub struct PlayGame {

}

impl PlayGame {
    pub fn new() -> Self {
        Self{}
    }

    pub fn tick(
        &mut self,
        _gl: &Gl,
        _ui: &imgui::Ui,
    ) -> super::ProgramMode {
        super::ProgramMode::PlayGame
    }
}