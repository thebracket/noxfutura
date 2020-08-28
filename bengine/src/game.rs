use crate::core::Core;

pub trait BEngineGame {
    fn init(&mut self);
    fn tick(&mut self, core: &mut Core) -> bool;
}
