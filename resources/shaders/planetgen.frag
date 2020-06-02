#version 450

layout(location=0) in vec4 v_color;
layout(location=1) in vec3 v_normal;
layout(location=0) out vec4 f_color;

void main() {
    vec3 lightDir = vec3(1, 0, 0);
    float diff = max(dot(v_normal, lightDir), 0.0);
    f_color = v_color * diff;
}