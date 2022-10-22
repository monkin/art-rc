precision mediump float;

/** 1, or 0 */
varying mediump float v_offset;
/** width in pixels */
varying mediump float v_width;

uniform highp vec2 u_resolution;

void main() {
    float opacity = 1.0 - smoothstep(v_width - 1.0, v_width, v_offset * v_width);
    gl_FragColor = vec4(opacity);
}