#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_uv;
layout(location=2) in float a_normal;
layout(location=3) in float a_material;
layout(location=4) in float a_texture;

layout(location=0) out vec3 v_world_pos;
layout(location=1) out flat float v_normal;
layout(location=2) out vec3 v_tint;
layout(location=3) out flat float v_texture;
layout(location=4) out vec2 v_uv;

layout(set=0, binding=0)
uniform Camera {
    mat4 u_view_proj;
};

layout(set = 1, binding = 0) buffer Palette {
    vec4[] palette;
};

void main() {
    gl_Position = u_view_proj * vec4(a_position, 1.0);
    v_world_pos = a_position;
    v_normal = a_normal;
    // TODO: Lookup the tint from the palette buffer
    int idx = int(a_material);
    v_tint = palette[idx].rgb;
    v_texture = a_texture;
    v_uv = a_uv;
}