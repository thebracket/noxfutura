#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_normal;
layout(location=2) in float a_material;
layout(location=3) in vec3 i_world_pos;
layout(location=4) in float i_scale;

layout(location=0) out vec3 v_tint;
layout(location=1) out vec3 v_normal;
layout(location=2) out vec3 v_world_pos;

layout(set=0, binding=0)
uniform Camera {
    mat4 u_view_proj;
};

layout(set = 1, binding = 0) buffer Palette {
    vec4[256] palette;
};

void main() {
    gl_Position = u_view_proj * vec4(((a_position * vec3(i_scale, i_scale, i_scale)) + i_world_pos + vec3(0.5, 0.0, 0.5)), 1.0);
    v_world_pos = i_world_pos;
    v_normal = normalize(a_normal);
    int idx = int(a_material);
    v_tint = palette[idx].rgb;
}