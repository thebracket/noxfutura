#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=0) out vec4 f_color;

layout(set = 0, binding = 0) uniform texture2D t_diffuse;
layout(set = 0, binding = 1) uniform sampler s_diffuse;
layout(set = 0, binding = 2) uniform texture2D t_normal;
layout(set = 0, binding = 3) uniform sampler s_normal;
layout(set = 0, binding = 4) uniform texture2D t_coords;
layout(set = 0, binding = 5) uniform sampler s_coords;

struct LightInfo {
    vec4 pos; // 4 contains the far_view
    vec4 color;
};

layout(set=1, binding=0) 
uniform Uniforms {
    vec4 camera_position;
    LightInfo lights[32];
};

layout(set = 2, binding = 0) buffer LightMap {
    uint[] light_bits;
};

int mapidx(vec3 position) {
    int zc = int(round(position.y));
    int yc = int(round(position.z));
    int xc = int(round(position.x));
    return (zc * 256 * 256) + (yc * 256) + xc;
}

void main() {
    vec4 albedo = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords);
    vec3 normal = normalize(texture(sampler2D(t_normal, s_normal), v_tex_coords).rgb);
    vec3 position = texture(sampler2D(t_coords, s_coords), v_tex_coords).rgb;

    vec3 light_output = vec3(0.0, 0.0, 0.0);
    for (int i=0; i<32; ++i) {
        uint flag = 1 << i;

        int idx = mapidx(position);
        if ( (light_bits[idx] & flag) > 0 && (i==0 || int(lights[i].pos.y) == int(round(position.y - 0.4))) ) {

            float radius = lights[i].pos.a;
            float distance = distance(lights[i].pos.xyz, position);
            if (radius > 0.0 && distance < radius) {
                float attenuation = radius > 64.0 ? 1.0 : 1.0 / (distance*distance);
                vec3 lightDir = normalize(lights[i].pos.rgb - position);
                float diff = max(dot(normal, lightDir), 0.0);
                light_output += diff * albedo.rgb * lights[i].color.rgb * attenuation;
            }

        }
    }
    light_output += vec3(0.05) * albedo.rgb; // Ambient component

    f_color = vec4(light_output, 1.0);
}