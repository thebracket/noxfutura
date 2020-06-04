#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec3 a_normal;

layout(location=0) out vec4 v_color;
layout(location=1) out vec3 v_normal;
layout(location=2) out vec3 v_frag_pos;
layout(location=3) out vec3 v_sun_pos;

layout(set=0, binding=0) 
uniform Uniforms {
    mat4 u_view_proj;
    vec3 u_sun_pos;
};

void main() {
    gl_Position = u_view_proj * vec4(a_position, 1.0);
    v_color = vec4(0.0, 1.0, 0.0, 1.0);
    v_normal = a_normal;
    v_frag_pos  = gl_Position.xyz;
    v_sun_pos = u_sun_pos;
}