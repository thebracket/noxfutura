#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=0) out vec4 f_color;

layout(set = 1, binding = 0) uniform texture2D t_diffuse;
layout(set = 1, binding = 1) uniform sampler s_diffuse;
layout(set = 1, binding = 2) uniform texture2D t_normal;
layout(set = 1, binding = 3) uniform sampler s_normal;
layout(set = 1, binding = 4) uniform texture2D t_pbr;
layout(set = 1, binding = 5) uniform sampler s_pbr;
layout(set = 1, binding = 6) uniform texture2D t_sun;
layout(set = 1, binding = 7) uniform sampler s_sun;
layout(set = 1, binding = 8) uniform texture2D t_coords;
layout(set = 1, binding = 9) uniform sampler s_coords;

// Uniform with the light position and transformation
layout(set=0, binding=0) 
uniform Uniforms {
    mat4 sun_view_proj;
    vec3 sun_pos;
    vec3 sun_color;
};

float shadowCalculation(vec3 world_pos) {
    vec4 fragPosLightSpace = sun_view_proj * vec4(world_pos.xyz, 1.0);
    //if (fragPosLightSpace.w <= 0.0) { discard; }
    const vec2 flip_correction = vec2(0.5, -0.5);
    vec3 light_local = vec3(
        fragPosLightSpace.xy * flip_correction / fragPosLightSpace.w + 0.5,
        fragPosLightSpace.z / fragPosLightSpace.w
    );
    float closestDepth = texture(sampler2D(t_sun, s_sun), light_local.xy).r;
    float currentDepth = light_local.z;
    float shadow = currentDepth - 0.005 > closestDepth ? 0.0 : 1.0;
    //if (shadow < 0.1) { discard; }
    return shadow;
}


void main() {
    vec3 world_pos = texture(sampler2D(t_coords, s_coords), v_tex_coords).rgb;
    vec3 normal = normalize(texture(sampler2D(t_normal, s_normal), v_tex_coords).rgb * 2.0 - 1.0);
    vec3 lightDir = normalize(vec3(0.1, 1.0, 0.0));
    float diff = max(dot(lightDir, normal), 0.1);
    float ao = texture(sampler2D(t_pbr, s_pbr), v_tex_coords).r;
    float rough = texture(sampler2D(t_pbr, s_pbr), v_tex_coords).g;
    float metal = texture(sampler2D(t_pbr, s_pbr), v_tex_coords).b;
    vec4 output_color = vec4(texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords).rgb * diff, 1.0);

    float shadow = shadowCalculation(world_pos);
    if (shadow < 0.1) { output_color = output_color * vec4(0.1, 0.1, 0.1, 1.0); }

    f_color = output_color;
}