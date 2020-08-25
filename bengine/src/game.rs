use crate::core::Core;
use crate::Initializer;

pub trait BEngineGame {
    fn init(&mut self, init: &mut Initializer);
    fn tick(&mut self, core: &mut Core) -> bool;
}
