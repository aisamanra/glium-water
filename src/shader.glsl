#version 140

uniform sampler2D water;
uniform float time;
uniform vec2 dims;

out vec4 color;

void main() {
  vec2 tc = gl_FragCoord.xy / dims;
  vec4 in_col = vec4(0.39, 0.58, 0.92, 1.0);

  float x = ( sin( time + 25 * tc.x + 30 * tc.y)
              + sin(-time + 20 * tc.x + 35 * tc.y + 1)
              ) / 2;
  float y = ( sin( time + 25 * tc.x + 30 * tc.y)
              + sin(-time + 16 * tc.x + 3 * tc.y + 1.5)
              ) / 2;

  vec2 off = vec2(x,y) * 0.08 + 1;
  vec2 wc  = 3 * (tc + 0.15 * off);

  vec4 light = texture(water, wc);
  vec4 dark  = texture(water, wc + 0.3);

  color = mix(mix(in_col, in_col * 0.9, dark), in_col * 2.2, light);
}
