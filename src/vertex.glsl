#version 140

in vec2 pt;

void main() {
  gl_Position = vec4(pt, 0.0, 1.0);
}
