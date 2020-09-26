#version 450
#extension GL_EXT_nonuniform_qualifier : require

layout(location=0) in vec3 v_world_pos;
layout(location=1) in vec3 v_normal;
layout(location=2) in vec3 v_tint;
layout(location=3) in flat float v_texture;
layout(location=4) in vec2 v_uv;

layout(location=0) out vec4 f_color;
layout(location=1) out vec4 f_normal;
layout(location=2) out vec4 f_coords;

layout(set = 2, binding = 0) uniform texture2D u_Textures[];
layout(set = 2, binding = 1) uniform sampler u_Sampler;

void main() {
    int texId = int(v_texture);
    vec3 texture_color = texId > -1 ? texture(sampler2D(u_Textures[texId], u_Sampler), v_uv).rgb : vec3(1.0, 1.0, 1.0);
    f_color = vec4(v_tint.rgb * texture_color, 1.0);
    f_normal = vec4(v_normal, 0.0);
    f_coords = vec4(v_world_pos, gl_FragCoord.z);
}