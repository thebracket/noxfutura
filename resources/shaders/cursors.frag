#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=1) flat in float v_tex_id;
layout(location=0) out vec4 f_color;

layout(set = 1, binding = 0) uniform texture2D t_cursor;
layout(set = 1, binding = 1) uniform sampler s_cursor;

void main() {
    vec3 color = texture(sampler2D(t_cursor, s_cursor), v_tex_coords).rgb;
    f_color = vec4(color, 0.1);
}