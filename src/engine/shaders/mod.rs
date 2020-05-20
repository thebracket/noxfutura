use super::Shader;

const VS_SRC: &str = "#version 100
precision mediump float;
attribute vec2 position;
attribute vec3 color;
varying vec3 v_color;
void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
}";

const FS_SRC: &str = "#version 100
precision mediump float;
varying vec3 v_color;
void main() {
    gl_FragColor = vec4(v_color, 1.0);
}";

pub fn load() -> Shader {
    Shader::new_selflock(VS_SRC, FS_SRC)
}