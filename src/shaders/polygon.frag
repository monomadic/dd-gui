#version 140

uniform vec2 u_resolution;
uniform vec4 u_color;
out vec4 color;

void main() {
//    vec2 st = gl_FragCoord.xy / u_resolution;
//    color = vec4(st.x, st.y, 0.0, 1.0);
    color = u_color;
}
