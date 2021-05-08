#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec2 a_uv;
layout(location=2) in float a_normal;
layout(location=3) in vec3 i_position;
layout(location=4) in float i_scale;

layout(location=0) out vec3 v_world_pos;
layout(location=1) out vec2 v_uv;
layout(location=2) out vec3 v_normal;

layout(set=0, binding=0)
uniform Camera {
    mat4 u_view_proj;
};

const vec3 normal_lut[10] = vec3[10](
    vec3(0.0, 1.0, 0.0), // Top
    vec3(0.0f,  0.0f, -1.0f), // Back
    vec3(0.0f,  0.0f, 1.0f), // Front
    vec3(-1.0f,  0.0f,  0.0f), // Left
    vec3(1.0f,  0.0f,  0.0f), // Right
    vec3(0.0f, -1.0f,  0.0f), // Bottom

    vec3(0.0, 0.5, 0.5), // Slope RampNS
    vec3(0.0, 0.5, -0.5), // Slope RampSN
    vec3(0.5, 0.5, 0.0), // Slope EW
    vec3(-0.5, 0.5, 0.0) // Slope WE
);

void main() {
    v_world_pos = (a_position * vec3(i_scale, i_scale, i_scale) + i_position);
    gl_Position = u_view_proj * vec4(v_world_pos, 1.0);
    v_uv = vec2(a_uv.x, 1.0 - a_uv.y);
    v_normal = normal_lut[int(a_normal)];
}