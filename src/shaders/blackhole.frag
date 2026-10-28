#version 100
precision highp float;
uniform vec2 Resolution;
uniform vec3 Eye;
uniform vec3 Forward;
uniform vec3 Right;
uniform vec3 Up;
uniform float Time;
uniform float Lensing;
uniform float Jets;
uniform float Density;
uniform float Steps;

const float PI = 3.14159265359;
const float INNER = 3.0;
const float OUTER = 10.0;

float hash(vec2 p) {
    return fract(sin(dot(p, vec2(127.1, 311.7))) * 43758.5453);
}
float noise(vec2 p) {
    vec2 i = floor(p), f = fract(p);
    f = f * f * (3.0 - 2.0 * f);
    return mix(mix(hash(i), hash(i + vec2(1.0, 0.0)), f.x),
               mix(hash(i + vec2(0.0, 1.0)), hash(i + 1.0), f.x), f.y);
}
float turbulence(vec2 p) {
    return 0.57 * noise(p) + 0.28 * noise(p * 2.07) + 0.15 * noise(p * 4.13);
}
vec3 sky(vec3 d) {
    // Directional cube projection avoids singularities at the celestial poles.
    vec3 a = abs(d);
    vec2 p;
    if (a.x > a.y && a.x > a.z) p = d.yz / a.x + vec2(sign(d.x) * 7.0, 0.0);
    else if (a.y > a.z) p = d.xz / a.y + vec2(sign(d.y) * 13.0, 5.0);
    else p = d.xy / a.z + vec2(sign(d.z) * 19.0, 11.0);
    vec3 color = vec3(0.0007, 0.0010, 0.0022);
    for (int layer = 0; layer < 2; layer++) {
        float scale = layer == 0 ? 145.0 : 310.0;
        vec2 cell = floor(p * scale);
        vec2 local = fract(p * scale);
        float seed = hash(cell + float(layer) * 43.0);
        vec2 center = vec2(hash(cell + 2.4), hash(cell + 9.2)) * 0.7 + 0.15;
        float star = exp(-dot(local - center, local - center) * 850.0);
        float active = step(0.986, seed);
        vec3 tint = mix(vec3(0.50, 0.68, 1.0), vec3(1.0, 0.78, 0.52), hash(cell + 5.0));
        color += tint * star * active * (0.65 + 2.5 * seed);
    }
    float band = exp(-pow((d.y + 0.28 * d.x) * 4.0, 2.0));
    color += vec3(0.004, 0.005, 0.009) * band * turbulence(d.xz * 7.0);
    return color;
}
vec3 disk(vec3 p, vec3 direction) {
    float r = length(p.xz);
    float angle = atan(p.z, p.x);
    float omega = 0.85 / pow(r, 1.5);
    // Periodic angular coordinates keep the turbulence continuous at +/-pi.
    float phase = angle - Time * omega;
    vec2 flow = vec2(cos(phase), sin(phase)) * (r * 2.5);
    float clouds = turbulence(flow + vec2(r * 2.0, r * 0.8));
    float streaks = 0.82 + 0.18 * sin(r * 23.0 + clouds * 7.0);
    float filaments = mix(0.65, 1.55, clouds) * streaks;
    float envelope = smoothstep(INNER, INNER + 0.55, r) * (1.0 - smoothstep(7.0, OUTER, r));
    vec3 tangent = normalize(vec3(-p.z, 0.0, p.x));
    float beta = sqrt(0.5 / max(r - 1.0, 1.0));
    float doppler = sqrt(1.0 - beta * beta) / (1.0 - beta * dot(tangent, -normalize(direction)));
    float shift = sqrt(max(1.0 - 1.0 / r, 0.0)) * doppler;
    float heat = pow(INNER / r, 0.75) * shift;
    vec3 temperature = mix(vec3(1.0, 0.12, 0.018), vec3(1.0, 0.78, 0.38), smoothstep(0.25, 0.8, heat));
    temperature = mix(temperature, vec3(0.78, 0.88, 1.0), smoothstep(0.9, 1.4, heat));
    float power = 5.4 * pow(INNER / r, 2.1) * pow(shift, 3.0);
    return temperature * power * filaments * envelope;
}
vec3 acceleration(vec3 p, float angular_momentum_squared) {
    float r2 = max(dot(p, p), 0.25);
    // Schwarzschild-inspired null-orbit force, in units where r_s = 1.
    return -1.5 * angular_momentum_squared * p / (r2 * r2 * sqrt(r2)) * Lensing;
}
void main() {
    vec2 screen = (2.0 * gl_FragCoord.xy - Resolution) / Resolution.y;
    vec3 direction = normalize(Forward + 0.41421356 * (screen.x * Right + screen.y * Up));
    vec3 p = Eye, velocity = direction;
    vec3 momentum = cross(p, velocity);
    float h2 = dot(momentum, momentum);
    vec3 light = vec3(0.0);
    float transmission = 1.0;
    bool captured = false;
    for (int i = 0; i < 260; i++) {
        if (float(i) >= Steps) break;
        float r = length(p);
        if (r < 1.0) { captured = true; break; }
        if (r > 45.0) break;
        float ds = clamp(r * 0.065, 0.035, 1.2) * (260.0 / Steps);
        vec3 force = acceleration(p, h2);
        vec3 next_p = p + velocity * ds + 0.5 * force * ds * ds;
        vec3 next_v = velocity + 0.5 * (force + acceleration(next_p, h2)) * ds;
        if (p.y * next_p.y < 0.0) {
            float fraction = p.y / (p.y - next_p.y);
            vec3 crossing = mix(p, next_p, fraction);
            float radius = length(crossing.xz);
            if (radius > INNER && radius < OUTER) {
                float opacity = mix(0.55, 0.97, Density);
                light += transmission * opacity * disk(crossing, velocity);
                transmission *= 1.0 - opacity;
            }
        }
        if (Jets > 0.5) {
            float height = abs(p.y);
            float width = 0.13 + height * 0.055;
            float beam = exp(-dot(p.xz, p.xz) / (width * width));
            float taper = smoothstep(1.2, 2.4, height) * (1.0 - smoothstep(5.0, 14.0, height));
            float knots = 0.65 + 0.35 * sin(height * 4.0 - Time * 3.0);
            light += transmission * vec3(0.13, 0.43, 1.0) * beam * taper * knots * ds * 0.7;
        }
        p = next_p;
        velocity = next_v;
        if (transmission < 0.005) break;
    }
    if (!captured) light += transmission * sky(normalize(velocity));
    // Encode HDR into an LDR render target. The post pass decodes before exposure.
    gl_FragColor = vec4(sqrt(light / (1.0 + light)), 1.0);
}
