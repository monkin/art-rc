precision highp float;

/* x, y, width, height */
uniform vec4 u_area;
uniform vec2 u_resolution;

attribute vec2 a_position;

varying mediump vec2 v_position;

void main() {
    v_position = a_position;
    gl_Position = vec4(
        (u_area.xy + u_area.zw * a_position) / u_resolution * vec2(2, -2) + vec2(-1, 1),
        0,
        1
    );
}