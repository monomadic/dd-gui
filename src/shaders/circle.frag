#version 140

uniform vec2  u_resolution;
uniform vec4  u_color;
uniform vec2  u_position;
uniform vec2  u_midpoint;
uniform float u_radius;

out vec4 color;

float smoothFloat(float r, float R) {
    return (1.0-smoothstep(R-1.0,R+1.0, r));
}

float circleLine(vec2 uv, vec2 center, float radius, float width) {
    float r = length(uv - center);
    return smoothFloat(r-width/2.0,radius) - smoothFloat(r+width/2.0,radius);
}

float circleFill(vec2 uv, vec2 center, float radius) {
    float r = length(uv - center);
    return smoothFloat(r, radius);
}

void main() {
	vec3 finalColor = vec3(0.0);
    vec2 uv = gl_FragCoord.xy;
    float borderWidth = 3.;
//    float padding = (borderWidth * 2.); // account for border so image doesn't chop off.
    vec2 c = u_position.xy;

    // line
    vec3 lineColor = vec3(0.3, 0.9, 0.9);
    float lineAlpha = circleLine(uv, c, u_radius, borderWidth);
    finalColor += lineColor * lineAlpha;

    // fill
    vec3 fillColor = vec3(0.4, 0.4, 0.9);
    float fillAlpha = (1.0 - lineAlpha) * circleFill(uv, c, u_radius);
    finalColor += fillColor * fillAlpha;

    color = vec4(finalColor, 1.0);
}
