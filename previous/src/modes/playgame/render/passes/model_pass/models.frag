#version 450

layout(location=0) in vec3 v_tint;
layout(location=1) in vec3 v_normal;
layout(location=2) in vec3 v_world_pos;

layout(location=0) out vec4 f_color;
layout(location=1) out vec4 f_normal;
layout(location=2) out vec4 f_coords;

void main() {
    f_color = vec4(v_tint.rgb, 1.0); // Use v_tint
    f_normal = vec4(v_normal, 1.0);
    f_coords = vec4(v_world_pos, gl_FragCoord.z);
}