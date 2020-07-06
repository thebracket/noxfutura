#version 450

layout(location=0) in vec3 v_tint;
layout(location=1) in vec3 v_world_pos;
layout(location=2) in vec2 v_uv;
layout(location=3) in float v_material;
layout(location=4) in mat3 v_tbn;

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

vec4 sample_material_exact(int diffuse_tex_index, vec2 uv) {
    vec2 diffuse = vec2(
        (float(diffuse_tex_index % 16) * 256.0),
        (float(diffuse_tex_index / 16) * 256.0)
    );
    vec2 terrain_uv = vec2(
        (uv.x + diffuse.x) / 4096.0,
        (uv.y + diffuse.y) / 4096.0
    );
    return textureLod(sampler2D(t_terrain, s_terrain), terrain_uv, 1.0);
}

void main() {
    int mat_base = int(v_material * 255.0);
    vec2 uv = vec2(
        clamp(fract(v_uv.x), 0.01, 0.99) * 256.0,
        clamp(fract(v_uv.y), 0.01, 0.99) * 256.0
    );

    vec4 terrain_color = sample_material_exact(mat_base, uv);
    vec3 tex_normal = sample_material(mat_base + 1, uv).rgb;
    tex_normal = normalize(tex_normal * 2.0 - 1.0);
    vec3 normal = normalize(v_tbn * tex_normal);
    vec3 pbr = sample_material(mat_base + 2, uv).rgb;

    f_color = terrain_color * vec4(v_tint, 1.0);
    f_normal = vec4(normal, 1.0);
    f_pbr = vec4(
        pbr.r, // AO
        pbr.g, // Rough
        pbr.b, // Metal
        0.0
    );
    f_coords = vec4(v_world_pos, 1.0);
}