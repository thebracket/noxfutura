use bracket_geometry::prelude::Point;

pub struct EngineContext {
    pub screen_size : Point
}

impl EngineContext {
    pub fn new() -> Self {
        Self {
            screen_size: Point::zero()
        }
    }
}