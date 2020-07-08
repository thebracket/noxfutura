#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in float a_normal;
layout(location=2) in vec3 a_tint;
layout(location=3) in vec3 i_position;
layout(location=4) in vec3 i_tint;
layout(location=5) in float i_rot;

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

mat4 rotationMatrix(vec3 axis, float angle)
{
    axis = normalize(axis);
    float s = sin(angle);
    float c = cos(angle);
    float oc = 1.0 - c;

    return mat4(oc * axis.x * axis.x + c,           oc * axis.x * axis.y - axis.z * s,  oc * axis.z * axis.x + axis.y * s,  0.0,
                oc * axis.x * axis.y + axis.z * s,  oc * axis.y * axis.y + c,           oc * axis.y * axis.z - axis.x * s,  0.0,
                oc * axis.z * axis.x - axis.y * s,  oc * axis.y * axis.z + axis.x * s,  oc * axis.z * axis.z + c,           0.0,
                0.0,                                0.0,                                0.0,                                1.0);
}

void main() {
    vec4 base_position = vec4(a_position - 0.5, 0.0);
    mat4 rotation = rotationMatrix(vec3(0.0, 1.0, 0.0), i_rot);
    vec3 rotated_position = (base_position * rotation).xyz;
    vec3 final_position = rotated_position + i_position + 0.5;

    v_normal = (vec4(normal_lut[int(a_normal)], 0.0) * rotation).xyz;
    gl_Position = u_view_proj * vec4(final_position, 1.0);
    v_world_pos = a_position + i_position;
    v_tint = a_tint * i_tint;
}