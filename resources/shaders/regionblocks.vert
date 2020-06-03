#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in vec4 a_color;
layout(location=2) in vec3 a_normal;

layout(location=0) out vec4 v_color;
layout(location=1) out vec3 v_normal;
layout(location=2) out vec3 v_frag_pos;

layout(set=0, binding=0) 
uniform Uniforms {
    mat4 u_view_proj;
    float rot_angle;
};

void main() {
    gl_Position = u_view_proj * vec4(a_position, 1.0);
    v_color = a_color;
    v_normal = a_normal;
    v_frag_pos  = a_position;
}