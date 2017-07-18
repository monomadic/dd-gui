#version 140

uniform mat4 ortho_projection;

in vec2 position;

void main() {
    vec4 position_matrix = vec4(0.1, 0.1, 0.0, 1.0);
    gl_Position = ortho_projection * vec4(position, 0.0, 1.0);
}
