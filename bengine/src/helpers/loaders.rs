use crate::{SHADERS, TEXTURES};

#[allow(dead_code)]
pub fn texture_from_bytes(bytes: &[u8], label: &str) -> usize {
    TEXTURES.write().load_texture_from_bytes(bytes, label)
}

#[allow(dead_code)]
pub fn texture_from_file(filename: &str, label: &str) -> usize {
    let image = image::open(filename).expect("Failed to open image");
    TEXTURES.write().load_texture_from_image(&image, label)
}

#[allow(dead_code)]
pub fn shader_from_bytes(
    vertex: wgpu::ShaderModuleSource,
    fragment: wgpu::ShaderModuleSource,
) -> (usize, usize) {
    let mut shaders = SHADERS.write();
    let vert_shader = shaders.register_include(vertex);
    let frag_shader = shaders.register_include(fragment);
    (vert_shader, frag_shader)
}
