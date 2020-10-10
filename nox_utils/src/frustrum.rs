use bengine::uv::{mat::Mat4, vec::{Vec3, Vec4}};

const LEFT: usize = 0;
const RIGHT: usize = 1;
const TOP: usize = 2;
const BOTTOM: usize = 3;
const BACK: usize = 4;
const FRONT: usize = 5;

pub struct Frustrum {
    planes: [Vec4; 6],
}

impl Frustrum {
    pub fn new() -> Self {
        Frustrum {
            planes: [
                (0.0, 0.0, 0.0, 0.0).into(),
                (0.0, 0.0, 0.0, 0.0).into(),
                (0.0, 0.0, 0.0, 0.0).into(),
                (0.0, 0.0, 0.0, 0.0).into(),
                (0.0, 0.0, 0.0, 0.0).into(),
                (0.0, 0.0, 0.0, 0.0).into(),
            ],
        }
    }

    pub fn update(&mut self, matrix: &Mat4) {
        self.planes[LEFT].x = matrix[0].w + matrix[0].x;
        self.planes[LEFT].y = matrix[1].w + matrix[1].x;
        self.planes[LEFT].z = matrix[2].w + matrix[2].x;
        self.planes[LEFT].w = matrix[3].w + matrix[3].x;

        self.planes[RIGHT].x = matrix[0].w - matrix[0].x;
        self.planes[RIGHT].y = matrix[1].w - matrix[1].x;
        self.planes[RIGHT].z = matrix[2].w - matrix[2].x;
        self.planes[RIGHT].w = matrix[3].w - matrix[3].x;

        self.planes[TOP].x = matrix[0].w - matrix[0].y;
        self.planes[TOP].y = matrix[1].w - matrix[1].y;
        self.planes[TOP].z = matrix[2].w - matrix[2].y;
        self.planes[TOP].w = matrix[3].w - matrix[3].y;

        self.planes[BOTTOM].x = matrix[0].w + matrix[0].y;
        self.planes[BOTTOM].y = matrix[1].w + matrix[1].y;
        self.planes[BOTTOM].z = matrix[2].w + matrix[2].y;
        self.planes[BOTTOM].w = matrix[3].w + matrix[3].y;

        self.planes[BACK].x = matrix[0].w + matrix[0].z;
        self.planes[BACK].y = matrix[1].w + matrix[1].z;
        self.planes[BACK].z = matrix[2].w + matrix[2].z;
        self.planes[BACK].w = matrix[3].w + matrix[3].z;

        self.planes[FRONT].x = matrix[0].w - matrix[0].z;
        self.planes[FRONT].y = matrix[1].w - matrix[1].z;
        self.planes[FRONT].z = matrix[2].w - matrix[2].z;
        self.planes[FRONT].w = matrix[3].w - matrix[3].z;

        self.planes.iter_mut().for_each(|p| {
            let length = f32::sqrt(p.x * p.x + p.y * p.y + p.z * p.z);
            *p /= length;
        });
    }

    pub fn check_sphere(&self, pos: &Vec3, radius: f32) -> bool {
        for p in self.planes.iter() {
            if (p.x * pos.x) + (p.y * pos.z) + (p.z * pos.y) + p.w <= -radius {
                return false;
            }
        }
        true
    }
}
