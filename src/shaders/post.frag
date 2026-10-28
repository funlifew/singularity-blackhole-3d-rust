#version 100
precision highp float;
varying vec2 uv;
uniform sampler2D Texture;
uniform vec2 Texel;
uniform float Exposure;
uniform float Bloom;
vec3 decode(vec2 p) {
    vec3 c = texture2D(Texture, clamp(p, Texel * 0.5, 1.0 - Texel * 0.5)).rgb;
    c *= c;
    return c / max(vec3(0.01), 1.0 - c);
}
vec3 aces(vec3 x) {
    return clamp((x * (2.51 * x + 0.03)) / (x * (2.43 * x + 0.59) + 0.14), 0.0, 1.0);
}
void main() {
    vec3 c = decode(uv), glow = vec3(0.0);
    for (int i = 0; i < 12; i++) {
        float angle = float(i) * 2.39996323;
        float radius = 2.0 + float(i) * 1.3;
        vec2 offset = vec2(cos(angle), sin(angle)) * Texel * radius;
        glow += max(decode(uv + offset) - 0.7, 0.0);
    }
    c += Bloom * glow / 12.0;
    c = pow(aces(c * Exposure), vec3(1.0 / 2.2));
    vec2 centered = uv * 2.0 - 1.0;
    c *= 1.0 - 0.12 * dot(centered, centered);
    gl_FragColor = vec4(c, 1.0);
}
