precision highp float;

uniform vec2 u_resolution;

attribute vec2 a_position;
attribute float a_offset;
attribute float a_width;

varying float v_width;
varying float v_offset;

void main() {
    v_width = a_width;
    v_offset = a_offset;
    gl_Position = vec4(a_position / u_resolution * 2.0 - 1.0, a_offset, 1);
}