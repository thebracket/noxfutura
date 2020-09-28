#version 450
#extension GL_EXT_nonuniform_qualifier : require

layout(location=0) in vec3 v_world_pos;
layout(location=1) in flat float v_normal;
layout(location=2) in vec3 v_tint;
layout(location=3) in flat float v_texture;
layout(location=4) in vec2 v_uv;

layout(location=0) out vec4 f_color;
layout(location=1) out vec4 f_normal;
layout(location=2) out vec4 f_coords;

layout(set = 2, binding = 0) uniform texture2D u_Textures[];
layout(set = 2, binding = 1) uniform sampler u_Sampler;

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

const vec3 tangent_lut[10] = vec3[10](
    vec3(1.0, 0.0, 0.0), // Top
    vec3(0.0f,  1.0f, 0.0f), // Back
    vec3(0.0f,  -1.0f, 0.0f), // Front
    vec3(0.0f,  0.0f,  -1.0f), // Left
    vec3(0.0f,  1.0f,  -1.0f), // Right
    vec3(1.0f, 0.0f,  0.0f), // Bottom

    vec3(0.5, 0.0, 0.0), // Slope RampNS
    vec3(0.5, 0.0, 0.0), // Slope RampNS
    vec3(0.0, 0.0, 0.5), // Slope RampEW
    vec3(0.0, 0.0, 0.5) // Slope RampWE
);

const vec3 bitangent_lut[10] = vec3[10](
    vec3(0.0, 0.0, 1.0), // Top
    vec3(1.0f,  0.0f, 0.0f), // Back
    vec3(-1.0f,  0.0f, 0.0f), // Front
    vec3(0.0f,  1.0f,  0.0f), // Left
    vec3(0.0f,  1.0f,  1.0f), // Right
    vec3(0.0f, 0.0f,  1.0f), // Bottom

    vec3(0.0, -0.25, -0.25), // Slope RampNS
    vec3(0.0, -0.25, 0.25), // Slope RampNS
    vec3(0.25, -0.25, 0.0), // Slope RampEW
    vec3(-0.25, -0.25, 0.0) // Slope RampEW
);

const mat3 TBN[10] = mat3[10](
    mat3(tangent_lut[0], bitangent_lut[0], normal_lut[0]),
    mat3(tangent_lut[1], bitangent_lut[1], normal_lut[1]),
    mat3(tangent_lut[2], bitangent_lut[2], normal_lut[2]),
    mat3(tangent_lut[3], bitangent_lut[3], normal_lut[3]),
    mat3(tangent_lut[4], bitangent_lut[4], normal_lut[4]),
    mat3(tangent_lut[5], bitangent_lut[5], normal_lut[5]),
    mat3(tangent_lut[6], bitangent_lut[6], normal_lut[6]),
    mat3(tangent_lut[7], bitangent_lut[7], normal_lut[7]),
    mat3(tangent_lut[8], bitangent_lut[8], normal_lut[8]),
    mat3(tangent_lut[9], bitangent_lut[9], normal_lut[9])
);

void main() {
    int texId = int(v_texture);
    vec3 texture_color = texId > -1 ? pow(texture(sampler2D(u_Textures[texId*2], u_Sampler), v_uv).rgb, vec3(1.0/2.2)) : vec3(1.0, 1.0, 1.0);
    int normalId = int(v_normal);
    f_color = vec4(v_tint.rgb * texture_color, 1.0);
    f_normal = texId > -1 ? vec4(TBN[normalId] * texture(sampler2D(u_Textures[(texId*2)+1], u_Sampler), v_uv).rgb, 1.0) : vec4(normal_lut[normalId], 0.0);
    f_coords = vec4(v_world_pos, gl_FragCoord.z);
}