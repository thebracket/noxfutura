#version 450

layout(location=0) in vec3 a_position;
layout(location=1) in float a_normal;
layout(location=2) in vec3 a_tint;
layout(location=3) in vec3 i_position;
layout(location=4) in vec3 i_tint;


layout(set=0, binding=0) 
uniform Uniforms {
    mat4 u_view_proj;
};

void main() {
    vec3 pos = a_position + i_position;
    gl_Position = u_view_proj * vec4(pos, 1.0);
}