#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_tex_coords;

layout(location=0) out vec2 v_tex_coords;
layout(location=1) flat out float v_tex_id;

layout(set=0, binding=0) 
uniform Uniforms {
    mat4 u_view_proj;
};

void main() {
    v_tex_coords = a_tex_coords.xy;
    v_tex_id = a_tex_coords.z;
    gl_Position = u_view_proj * vec4(a_position, 1.0);
}