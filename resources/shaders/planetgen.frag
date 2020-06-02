#version 450

layout(location=0) in vec4 v_color;
layout(location=1) in vec3 v_normal;
layout(location=2) in vec3 v_frag_pos;
layout(location=0) out vec4 f_color;

void main() {
    vec3 lightPos = vec3(2.0, 2.0, 1.0);
    vec3 lightDir = normalize(lightPos - v_frag_pos);
    float diff = max(dot(lightDir, v_normal), 0.0);
    f_color = vec4(v_color.rgb * diff, 1.0);
}