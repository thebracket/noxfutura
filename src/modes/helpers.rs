use crate::opengl::*;
use super::resources::SharedResources;

pub fn render_menu_background(gl: &Gl, resources: &SharedResources) {
    gl_error(gl);
    resources.quad_vao.as_ref().unwrap().draw_elements(
        gl, 
        resources.quad_shader.as_ref().unwrap(), 
        resources.background_image.as_ref().unwrap()
    );
    gl_error(gl);
}
