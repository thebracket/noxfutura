#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in float a_normal;
layout(location=2) in vec3 a_tint;
layout(location=3) in vec3 i_position;
layout(location=4) in vec3 i_tint;

layout(location=0) out vec3 v_normal;
layout(location=1) out vec3 v_world_pos;
layout(location=2) out vec3 v_tint;

layout(set=0, binding=0) 
uniform Uniforms {
    mat4 u_view_proj;
};

const vec3 normal_lut[6] = vec3[6](
    vec3(0.0, 1.0, 0.0), // Top
    vec3(0.0f,  0.0f, -1.0f), // Back
    vec3(0.0f,  0.0f, 1.0f), // Front
    vec3(-1.0f,  0.0f,  0.0f), // Left
    vec3(1.0f,  0.0f,  0.0f), // Right
    vec3(0.0f, -1.0f,  0.0f) // Bottom
);

void main() {
    vec3 pos = a_position + i_position;
    gl_Position = u_view_proj * vec4(pos, 1.0);
    v_normal = normal_lut[int(a_normal)];
    v_world_pos = a_position + i_position;
    float gamma = 2.2;
    v_tint = pow(a_tint * i_tint, vec3(gamma));
}