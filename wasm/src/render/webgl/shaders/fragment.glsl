#version 300 es

precision mediump float;

uniform vec2 u_texture_atlas_size;
uniform sampler2D u_texture;

in vec2 v_texcoord;

out vec4 color;

void main() {
    vec2 clip_space = v_texcoord / u_texture_atlas_size;
    color = texture(u_texture, clip_space);
}
