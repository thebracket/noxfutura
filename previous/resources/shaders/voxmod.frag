#version 450

layout(location=0) in vec3 v_normal;
layout(location=1) in vec3 v_world_pos;
layout(location=2) in vec3 v_tint;
layout(location=3) in vec3 v_model_pos;

layout(location=0) out vec4 f_color;
layout(location=1) out vec4 f_normal;
layout(location=2) out vec4 f_pbr;
layout(location=3) out vec4 f_coords;


void main() {
    f_color = vec4(v_tint, 1.0);
    f_normal = vec4(v_normal, 1.0);
    f_coords = vec4(v_world_pos, gl_FragCoord.z);
    f_pbr = vec4(1.0, 1.0, 0.0, 0.0);
}