use egui::CtxRef;
use winit::dpi::PhysicalSize;

pub enum TickResult {
    Continue,
    MainMenu,
    WorldGen,
    MakePlanet { seed: u64 },
    Quit,
}

pub trait GameMode {
    /// Pre_init runs at the very start-up and is used to indicate essentials
    /// that must be loaded to get things going.
    fn pre_init(&mut self) {}

    fn init(&mut self) {}
    fn activate(&mut self) {}
    fn tick(
        &mut self,
        _size: PhysicalSize<u32>,
        _egui: &CtxRef,
        _swap_chain_texture: &wgpu::SwapChainTexture,
    ) -> TickResult {
        TickResult::Continue
    }
    fn deactivate(&mut self) {}
    fn world_gen_params(&mut self, _seed: u64) {}
}
