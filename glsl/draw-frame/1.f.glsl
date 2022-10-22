precision mediump float;

uniform sampler2D u_source;

varying mediump vec2 v_position;

void main() {
    gl_FragColor = texture2D(u_source, v_position);
}
