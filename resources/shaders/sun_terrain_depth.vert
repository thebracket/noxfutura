#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in float a_normal;
layout(location=2) in vec2 a_uv;
layout(location=3) in float a_material;

layout(set=0, binding=0)
uniform Uniforms {
    mat4 u_view_proj;
    vec3 sun_pos;
};

void main() {
    gl_Position = u_view_proj * vec4(a_position, 1.0);
}