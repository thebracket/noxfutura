#version 430 core

in VS_OUT {
    in vec2 tex_coords;
} fs_in;

out vec4 FragColor;

uniform sampler2D QuadTex;

void main() {
    FragColor = texture(QuadTex, fs_in.tex_coords);
}