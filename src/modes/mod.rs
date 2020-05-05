mod resources;
use resources::SharedResources;
mod loader;
use loader::Loader;
mod helpers;

pub enum ProgramMode {
    Loader,
}

pub struct Program {
    mode: ProgramMode,
    resources: SharedResources,
    loader: Loader,
}

impl Program {
    pub fn new() -> Self {
        Self {
            mode: ProgramMode::Loader,
            resources: SharedResources::new(),
            loader: Loader::new(),
        }
    }

    pub fn init(&mut self, context: &mut crate::engine::Context) {
        self.resources.init(context);
    }

    pub fn tick(
        &mut self,
        context: &mut crate::engine::Context,
        frame: &wgpu::SwapChainOutput,
        _depth_id: usize,
        imgui: &imgui::Ui,
    ) {
        match self.mode {
            ProgramMode::Loader => {
                self.mode = self
                    .loader
                    .tick(&self.resources, frame, context, imgui)
            }
        }
    }
}
