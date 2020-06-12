#version 450

layout(location=0) in vec3 v_normal;
layout(location=1) in vec3 v_frag_pos;
layout(location=2) in vec3 v_sun_pos;
layout(location=3) in vec3 v_world_pos;
layout(location=4) in vec2 v_uv;
layout(location=5) in float v_material;

layout(location=0) out vec4 f_color;
layout(location=1) out vec4 f_normal;
layout(location=2) out vec4 f_pbr;
layout(location=3) out vec4 f_coords;

layout(set = 1, binding = 0) uniform texture2D t_terrain;
layout(set = 1, binding = 1) uniform sampler s_terrain;

vec4 sample_material(int diffuse_tex_index, vec2 uv) {
    vec2 diffuse = vec2(
        (float(diffuse_tex_index % 16) * 256.0),
        (float(diffuse_tex_index / 16) * 256.0)
    );
    vec2 terrain_uv = vec2(
        (uv.x + diffuse.x) / 4096.0,
        (uv.y + diffuse.y) / 4096.0
    );
    return texture(sampler2D(t_terrain, s_terrain), terrain_uv);
}

void main() {
    int mat_base = int(v_material * 255.0);
    int diffuse_tex_index = mat_base + 4; // +4
    vec2 uv = vec2(
        fract(v_uv.x) * 256.0,
        fract(v_uv.y) * 256.0
    );

    vec4 terrain_color = sample_material(diffuse_tex_index, uv);

    f_color = terrain_color;
    f_normal = vec4(v_normal, 1.0);
    f_pbr = vec4(1.0, 0.0, 1.0, 1.0);
    f_coords = vec4(v_world_pos, 1.0);
}