#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in float a_normal;
layout(location=2) in vec2 a_uv;
layout(location=3) in float a_material;

layout(location=0) out vec3 v_normal;
layout(location=1) out vec3 v_frag_pos;
layout(location=2) out vec3 v_world_pos;
layout(location=3) out vec2 v_uv;
layout(location=4) out float v_material;
layout(location=5) out mat3 v_tbn;

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

const vec3 tangent_lut[6] = vec3[6](
    vec3(1.0, 0.0, 0.0), // Top
    vec3(0.0f,  1.0f, 0.0f), // Back
    vec3(0.0f,  -1.0f, 0.0f), // Front
    vec3(0.0f,  0.0f,  -1.0f), // Left
    vec3(0.0f,  1.0f,  -1.0f), // Right
    vec3(1.0f, 0.0f,  0.0f) // Bottom
);

const vec3 bitangent_lut[6] = vec3[6](
    vec3(0.0, 0.0, 1.0), // Top
    vec3(1.0f,  0.0f, 0.0f), // Back
    vec3(-1.0f,  0.0f, 0.0f), // Front
    vec3(0.0f,  1.0f,  0.0f), // Left
    vec3(0.0f,  1.0f,  1.0f), // Right
    vec3(0.0f, 0.0f,  1.0f) // Bottom
);

const mat3 TBN[6] = mat3[6](
    mat3(tangent_lut[0], bitangent_lut[0], normal_lut[0]),
    mat3(tangent_lut[1], bitangent_lut[1], normal_lut[1]),
    mat3(tangent_lut[2], bitangent_lut[2], normal_lut[2]),
    mat3(tangent_lut[3], bitangent_lut[3], normal_lut[3]),
    mat3(tangent_lut[4], bitangent_lut[4], normal_lut[4]),
    mat3(tangent_lut[5], bitangent_lut[5], normal_lut[5])
);

void main() {
    gl_Position = u_view_proj * vec4(a_position, 1.0);
    v_normal = normal_lut[int(a_normal)];
    v_frag_pos  = gl_Position.xyz;
    v_world_pos = vec3(a_position.x / 256.0, a_position.z / 256.0, a_position.y / 256.0);
    v_uv = a_uv;
    v_material = a_material;
    v_tbn = TBN[int(a_normal)];
}