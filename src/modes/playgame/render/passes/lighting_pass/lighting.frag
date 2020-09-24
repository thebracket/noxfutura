#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;
layout(set = 0, binding = 2) uniform texture2D t_normal;
layout(set = 0, binding = 3) uniform sampler s_normal;

void main() {
    vec3 normal = texture(sampler2D(t_normal, s_normal), v_tex_coords).rgb;
    vec3 light_pos = vec3(128.0, 512.0, 0.1);
    vec3 light_dir = normalize(light_pos);
    float diff = max(dot(normal, light_dir), 0.0);

    vec4 bc = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords);
    f_color = bc * diff;
}