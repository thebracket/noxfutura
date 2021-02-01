#version 450

layout(location=0) in vec3 v_world_pos;
layout(location=1) in vec2 v_uv;
layout(location=2) in vec3 v_normal;

layout(location=0) out vec4 f_color;
layout(location=1) out vec4 f_normal;

layout(set = 0, binding = 1) uniform texture2D t_grass;
layout(set = 0, binding = 2) uniform sampler s_grass;

void main() {
    vec4 base_color = texture(sampler2D(t_grass, s_grass), v_uv);
    if (base_color.a < 1.0) {
        discard;
    }
    f_color = base_color;
    f_normal = vec4(v_normal, 1.0);
}