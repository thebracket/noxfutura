#version 450

layout(location=0) in vec2 a_position;
layout(location=1) in vec2 a_tex_coords;

layout(location=0) out vec2 v_tex_coords;

struct LightInfo {
    vec4 pos; // 4 contains the far_view
    vec4 color;
};

layout(set=1, binding=0) 
uniform Uniforms {
    vec4 camera_position;
    LightInfo lights[32];
};

void main() {
    v_tex_coords = a_tex_coords;
    gl_Position = vec4(a_position, 0.0, 1.0);
}