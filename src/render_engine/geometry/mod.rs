use crate::opengl::*;
use super::Primitive;
mod floor;
pub use floor::*;
mod cube;
pub use cube::*;

pub fn all_region_primitives(primitives: Vec<Primitive>, vb: &mut VertexArray) {
    vb.vertex_buffer.clear();
    primitives.iter().for_each(|p| {
        match *p {
            Primitive::Cube{x, y, z, w, h, d} => {
                //println!("{},{},{} .. {},{},{}", x, y, z, w, h, d);
                //add_cube(x, y, z, w, h, d, vb);
                add_cube_geometry(vb, x, y, z, w, h, d);
            }
        }
    });
}