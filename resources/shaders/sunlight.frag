#version 450

layout(location=0) in vec2 v_tex_coords;
layout(location=0) out vec4 f_color;

layout(set = 1, binding = 0) uniform texture2D t_diffuse;
layout(set = 1, binding = 1) uniform sampler s_diffuse;
layout(set = 1, binding = 2) uniform texture2D t_normal;
layout(set = 1, binding = 3) uniform sampler s_normal;
layout(set = 1, binding = 4) uniform texture2D t_pbr;
layout(set = 1, binding = 5) uniform sampler s_pbr;
layout(set = 1, binding = 6) uniform texture2D t_coords;
layout(set = 1, binding = 7) uniform sampler s_coords;

layout(set = 2, binding = 0) buffer LightMap {
    uint[] light_bits;
};

// Uniform with light data
struct LightInfo {
    vec4 pos; // 4 contains the far_view
    vec4 color;
};

layout(set=0, binding=0) 
uniform Uniforms {
    vec4 camera_position;
    LightInfo lights[32];
};

#define PI 3.1415926

float DistributionGGX(vec3 N, vec3 H, float roughness)
{
    float a = roughness*roughness;
    float a2 = a*a;
    float NdotH = max(dot(N, H), 0.0);
    float NdotH2 = NdotH*NdotH;

    float nom   = a2;
    float denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return nom / denom;
}

float GeometrySchlickGGX(float NdotV, float roughness)
{
    float r = (roughness + 1.0);
    float k = (r*r) / 8.0;

    float nom   = NdotV;
    float denom = NdotV * (1.0 - k) + k;

    return nom / denom;
}

float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness)
{
    float NdotV = max(dot(N, V), 0.0);
    float NdotL = max(dot(N, L), 0.0);
    float ggx2 = GeometrySchlickGGX(NdotV, roughness);
    float ggx1 = GeometrySchlickGGX(NdotL, roughness);

    return ggx1 * ggx2;
}

vec3 fresnelSchlick(float cosTheta, vec3 F0)
{
    return F0 + (1.0 - F0) * pow(1.0 - cosTheta, 5.0);
}

vec3 CalculateLightOutput(vec3 albedo, vec3 N, vec3 V, vec3 F0, vec3 L, vec3 radiance, float roughness, float metallic) {
    vec3 H = normalize(V + L);
    float NDF = DistributionGGX(N, H, roughness);
    float G   = GeometrySmith(N, V, L, roughness);
    vec3 F    = fresnelSchlick(max(dot(H, V), 0.0), F0);

    vec3 nominator    = NDF * G * F;
    float denominator = 4 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0) + 0.001; // 0.001 to prevent divide by zero.
    vec3 specular = nominator / denominator;

    // kS is equal to Fresnel
    vec3 kS = F;
    // for energy conservation, the diffuse and specular light can't
    // be above 1.0 (unless the surface emits light); to preserve this
    // relationship the diffuse component (kD) should equal 1.0 - kS.
    vec3 kD = vec3(1.0) - kS;
    // multiply kD by the inverse metalness such that only non-metals
    // have diffuse lighting, or a linear blend if partly metal (pure metals
    // have no diffuse light).
    kD *= 1.0 - metallic;

    // scale light by NdotL
    float NdotL = max(dot(N, L), 0.0);

    return (kD * (albedo / PI) + specular) * NdotL;
    //return (kD * (albedo / PI) + specular);
}

vec3 GameLight(float far_plane, vec3 albedo, vec3 N, vec3 V, vec3 F0, float roughness, float metallic, vec3 light_position, vec3 position, vec3 light_color, float distance) {
    vec3 L = far_plane < 512.0 ? normalize(light_position - position) : normalize(light_position);
    float attenuation = far_plane > 64.0 ? 1.0 : 1.0 / (distance * 1.0);
    vec3 radiance = light_color * attenuation;

    // For simple light output calculation
    //float diff = max(dot(L, N), 0.0);
    //return albedo * diff;

    return CalculateLightOutput(albedo, N, V, F0, L, radiance, roughness, metallic) * radiance;
}

int mapidx(vec3 position) {
    int zc = int(round(position.y));
    int yc = int(round(position.z));
    int xc = int(round(position.x));
    return (zc * 256 * 256) + (yc * 256) + xc;
}

const vec3 samples[64]=vec3[64](
vec3(-0.047929014979204, -0.438413764902785, 0.929426961233746),
vec3(-0.269709204592704, 0.950515041224427, 0.206353989610903),
vec3(0.115462048337919, -0.973596029128394, -0.908342458086027),
vec3(0.0806885698758024, 0.925705718195549, -0.775657147370491),
vec3(-0.818854025110074, 0.552612993093465, -0.837888646794868),
vec3(0.917812085170002, 0.511669141026135, 0.482702973208565),
vec3(0.860502954135615, -0.643916628904123, -0.59160008969527),
vec3(0.4927422379035, 0.664258196344865, 0.351500938183611),
vec3(0.456449260085052, 0.451071517042271, 0.197256048385459),
vec3(-0.0770890598977241, -0.731329924012184, 0.256627452486744),
vec3(0.762439363558385, 0.289929235346325, -0.980721506556118),
vec3(0.805745881028283, 0.981082720275267, 0.959898172252732),
vec3(0.987773624658742, 0.894268979850048, 0.532481806436596),
vec3(0.991634111497466, 0.70068819104743, -0.763940778113785),
vec3(0.951780368759517, -0.0768451894573428, -0.500051180049417),
vec3(-0.240563757243764, 0.712237722010253, 0.174227118287188),
vec3(0.252475956057157, 0.853581860825418, 0.6421901210045),
vec3(-0.431675103916176, 0.641885146287785, 0.141548215387181),
vec3(-0.576182500075215, -0.516059089610147, -0.711348565943524),
vec3(0.716071723993237, 0.388852186242922, 0.812300858012269),
vec3(-0.935177913235506, 0.838241045743039, -0.838464823718813),
vec3(0.776158396358163, 0.0929938174762905, 0.292905676123544),
vec3(0.806935350679827, -0.838175281740873, 0.0393023892256006),
vec3(0.00745805126898125, -0.934154285074698, 0.605661123442571),
vec3(0.382660761354565, -0.77356647488387, -0.640886274395705),
vec3(0.61448898485048, 0.988673995403063, 0.116667312373815),
vec3(-0.909105247135674, -0.706164038343898, -0.481175743028916),
vec3(0.0462535540686926, 0.268954108655582, -0.902370086213705),
vec3(-0.228520638585718, 0.0409157975495393, -0.552664589745119),
vec3(0.571791567764772, 0.77897243466913, 0.469908848804465),
vec3(0.580044974780119, 0.837263924648454, 0.431036646984342),
vec3(-0.0489893534003558, 0.996032016268539, 0.768380880706172),
vec3(-0.558274395079814, 0.549639131300883, 0.27837004455577),
vec3(0.167467224381758, 0.0108904739165647, -0.790657199475527),
vec3(0.04403383418187, 0.895698401441043, -0.842476656419072),
vec3(0.938746585792893, -0.956280883354866, 0.837286858731445),
vec3(-0.443537157641154, -0.676352769266339, -0.128302908384674),
vec3(-0.87418601714471, 0.322312905774096, -0.759712795440762),
vec3(-0.724544917831573, -0.00221919553867789, -0.0967711298837752),
vec3(-0.975161909900947, 0.0565221001195153, 0.183158396566239),
vec3(0.818283460579766, -0.623341131409574, -0.731885250080112),
vec3(-0.637799055035528, -0.276806048820645, -0.426942824099501),
vec3(-0.516780444592215, 0.652402509594556, -0.0845697275223216),
vec3(0.455657960327755, -0.00613636484725588, 0.147927560100475),
vec3(0.636419369995088, -0.956138133925783, 0.0624363693983658),
vec3(0.276018297299378, 0.510623144362949, 0.716888975171009),
vec3(-0.826548283954511, -0.144072268338394, 0.596106303710207),
vec3(-0.794919454804488, 0.861427997013716, 0.131281364753063),
vec3(0.249229160214675, -0.507723697343464, 0.441255709612241),
vec3(0.0840510032785455, 0.401298733738271, 0.578262938348589),
vec3(-0.873061136588521, -0.742967173096409, 0.374191724415234),
vec3(-0.218813256326754, -0.270346983742737, -0.111649232343958),
vec3(-0.332886091609431, 0.569517305476091, 0.150298638205109),
vec3(-0.988458778680371, 0.887796161547977, 0.991071520382182),
vec3(-0.131937976781313, 0.26141419201547, 0.391276265635145),
vec3(0.976950409200295, 0.256124412582709, 0.844499972646254),
vec3(-0.716166136917728, -0.834360097309523, -0.986826448508014),
vec3(-0.764498859094076, -0.677069968946773, -0.991087930604148),
vec3(0.882030606279058, -0.390102148278822, 0.0877066348029119),
vec3(-0.339327945617814, -0.310451413094155, -0.563799889980217),
vec3(-0.990303448980524, -0.386055397295728, -0.461347742783093),
vec3(0.70775976348724, 0.772126136572357, 0.854851706342142),
vec3(-0.355966233058895, -0.146968676804214, 0.265689179962858),
vec3(-0.145761658046816, 0.373878060261686, 0.65622220492727)
);

void main() {
    vec4 bc = texture(sampler2D(t_diffuse, s_diffuse), v_tex_coords);
    vec3 albedo =bc.rgb;
    vec4 raw_pos = texture(sampler2D(t_coords, s_coords), v_tex_coords);
    vec3 position = raw_pos.rgb;
    vec3 normal = texture(sampler2D(t_normal, s_normal), v_tex_coords).rgb;

    vec3 material_lookup = texture(sampler2D(t_pbr, s_pbr), v_tex_coords).rgb;
    float ao = material_lookup.r;
    float rough = material_lookup.g;
    float metal = material_lookup.b;

    vec3 V = normalize(camera_position.xyz - position);
    vec3 F0 = mix(vec3(0.04), albedo, metal);

    vec3 light_output = vec3(0.0, 0.0, 0.0);
    for (int i=0; i<32; ++i) {
        uint flag = 1 << i;

        int idx = mapidx(position);
        if ( (light_bits[idx] & flag) > 0 ) {

            float radius = lights[i].pos.a;
            float distance = distance(lights[i].pos.xyz, position);
            if (radius > 0.0 && distance < radius) {
                light_output += GameLight(radius, albedo, normal, V, F0, rough, metal, lights[i].pos.xyz, position, lights[i].color.xyz, distance);
            }

        }
    }
    light_output += vec3(0.05) * albedo * ao; // Ambient component

    // Darken by difference with camera height
    float y_difference = 1.0 - ((camera_position.y - position.y) * 0.025);
    light_output *= y_difference;

    // SSAO. Again.
    float SSAO = 0.0;
    const float kernelSize = 64.0;
    const float radius = 0.005;
    vec3 FragPos = vec3(v_tex_coords.xy, raw_pos.a);
    float occlusion = 0;
    for (int i=0; i<kernelSize; ++i) {
        vec2 mysample = FragPos.xy + (samples[i].xy * radius);
        float sampleDepth = texture(sampler2D(t_coords, s_coords), mysample).a;
        float rangeCheck = smoothstep(0.0, 1.0, radius / abs(FragPos.z - sampleDepth));
        occlusion += (sampleDepth >= FragPos.z ? 1.00 : 0.0) * rangeCheck;
    }
    SSAO = clamp((occlusion / kernelSize), 0.0, 1.0);

    light_output *= vec3(SSAO, SSAO, SSAO);

    f_color = vec4(light_output, 1.0);
    //f_color = vec4(SSAO, SSAO, SSAO, 1.0);
}