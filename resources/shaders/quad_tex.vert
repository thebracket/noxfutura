#version 430 core

layout (location = 0) in vec2 aPos;
layout (location = 1) in vec2 aTexCoords;

out VS_OUT {
    out vec2 tex_coords;
} vs_out;


void main() 
{
    vs_out.tex_coords = aTexCoords;
    gl_Position = vec4(aPos, 1.0, 1.0);
}