#version 450

layout(location=0) in vec3 v_normal;
layout(location=1) in vec3 v_frag_pos;
layout(location=2) in vec3 v_sun_pos;
layout(location=0) out vec4 f_color;

void main() {
    vec3 lightDir = normalize(v_sun_pos - v_frag_pos);
    float diff = max(dot(lightDir, v_normal), 0.05);
    f_color = vec4(vec3(0.0, 1.0, 0.0) * diff, 1.0);
}