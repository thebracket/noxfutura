#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_normal;
layout(location=2) in float a_material;

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
    gl_Position = u_view_proj * vec4((a_position / 100.0) + vec3(128.0, 256.0, 128.0), 1.0);
    v_world_pos = a_position;
    v_normal = a_normal;
    int idx = int(a_material);
    v_tint = palette[idx].rgb;
}