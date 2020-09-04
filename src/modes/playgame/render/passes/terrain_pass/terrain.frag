#version 450

layout(location=0) in vec3 v_world_pos;
layout(location=1) in vec3 v_normal;
layout(location=2) in vec3 v_tint;

layout(location=0) out vec4 f_color;

void main() {
    f_color = vec4(v_tint.rgb, 1.0);
}