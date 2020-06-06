#version 450

layout(location=0) in vec3 v_normal;
layout(location=1) in vec3 v_frag_pos;
layout(location=2) in vec3 v_sun_pos;
layout(location=3) in vec3 v_world_pos;
layout(location=4) in vec2 v_uv;
layout(location=0) out vec4 f_color;

layout(set = 1, binding = 0) uniform texture3D t_matinfo;
layout(set = 1, binding = 1) uniform sampler s_matinfo;

layout(set = 2, binding = 0) uniform texture2D t_terrain[212];
layout(set = 2, binding = 1) uniform sampler s_terrain;

void main() {
    vec4 mat_tint = texture(sampler3D(t_matinfo, s_matinfo), v_world_pos);
    //vec4 terrain_color = texture(sampler2D(t_terrain[int(mat_tint.a * 255.0)], s_terrain), v_uv);
    vec4 tinted = mat_tint;

    vec3 lightDir = normalize(v_sun_pos - v_frag_pos);
    float diff = max(dot(lightDir, v_normal), 0.05);
    f_color = vec4(tinted.rgb * diff, 1.0);
}