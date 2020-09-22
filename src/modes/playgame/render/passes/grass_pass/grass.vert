#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_uv;

layout(location=0) out vec3 v_world_pos;
layout(location=1) out vec2 v_uv;

layout(set=0, binding=0)
uniform Camera {
    mat4 u_view_proj;
};

void main() {
    gl_Position = u_view_proj * vec4(a_position, 1.0);
    v_world_pos = a_position;
    v_uv = vec2(a_uv.x, 1.0 - a_uv.y);
}