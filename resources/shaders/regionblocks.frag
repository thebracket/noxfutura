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

void main() {
    int mat_base = int(v_material * 255.0);
    //int mat_base = 0;
    int diffuse_tex_index = mat_base + 4; // +4
    vec2 uv = vec2(
        clamp(fract(v_uv.x),0.1,0.9) * 256.0,
        clamp(fract(v_uv.y),0.1,0.9) * 256.0
    );
    vec2 diffuse = vec2(
        (float(diffuse_tex_index % 16) * 256.0),
        (float(diffuse_tex_index / 16) * 256.0)
    );

    vec2 terrain_uv = vec2(
        (uv.x + diffuse.x) / 4096.0,
        (uv.y + diffuse.y) / 4096.0
    );

    vec4 terrain_color = texture(sampler2D(t_terrain, s_terrain), terrain_uv);
    vec4 tinted = terrain_color;

    vec3 lightDir = normalize(v_sun_pos - v_frag_pos);
    float diff = max(dot(lightDir, v_normal), 0.1);
    f_color = vec4(tinted.rgb * diff, 1.0);
    f_normal = vec4(v_normal, 1.0);
    f_pbr = vec4(1.0, 0.0, 1.0, 1.0);
    f_coords = vec4(v_world_pos, 1.0);
}