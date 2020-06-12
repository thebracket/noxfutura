#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;

layout(set = 0, binding = 2) uniform texture2D t_normal;
layout(set = 0, binding = 3) uniform sampler s_normal;

layout(set = 0, binding = 4) uniform texture2D t_pbr;
layout(set = 0, binding = 5) uniform sampler s_pbr;

void main() {
    vec3 normal = normalize(texture(sampler2D(t_normal, s_normal), v_tex_coords).rgb * 2.0 - 1.0);
    vec3 lightDir = normalize(vec3(0.7, 1.0, 0.0));
    float diff = max(dot(lightDir, normal), 0.1);
    float ao = texture(sampler2D(t_pbr, s_pbr), v_tex_coords).g;
    f_color = vec4(texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords).rgb * diff * ao, 1.0);
}