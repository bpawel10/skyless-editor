#version 300 es

precision mediump float;

uniform float u_tile_size;
uniform vec2 u_resolution;
uniform mat3 u_transformation;

in vec2 a_position;
in vec2 a_texcoord;

out vec2 v_texcoord;

void main() {
    vec2 pixel_position = a_position * vec2(u_tile_size);
    gl_Position = vec4((u_transformation * vec3(pixel_position, 1)).xy, 0, 1);
    v_texcoord = a_texcoord;
}
