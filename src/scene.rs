use std::f32::consts::TAU;

use macroquad::models::{Mesh, Vertex, draw_mesh};
use macroquad::prelude::*;

use crate::rng::Rng;

const INNER_RADIUS: f32 = 1.34;
const OUTER_RADIUS: f32 = 5.7;

const RADIAL_STEPS: usize = 44;
const ANGULAR_STEPS: usize = 168;

struct Particle {
    radius: f32,
    phase: f32,
    height: f32,
    brightness: f32,
    length: f32,
}

pub struct Scene {
    disk: Mesh,
    particles: Vec<Particle>,
}

impl Scene {
    pub fn new() -> Self {
        let vertex_count = (RADIAL_STEPS + 1) * (ANGULAR_STEPS + 1);

        assert!(vertex_count < u16::MAX as usize);

        let mut indices = Vec::with_capacity(RADIAL_STEPS * ANGULAR_STEPS * 6);

        for row in 0..RADIAL_STEPS {
            for col in 0..ANGULAR_STEPS {
                let top_left = row * (ANGULAR_STEPS + 1) + col;

                let bottom_left = top_left + ANGULAR_STEPS + 1;

                indices.extend_from_slice(&[
                    top_left as u16,
                    bottom_left as u16,
                    (top_left + 1) as u16,
                    (top_left + 1) as u16,
                    bottom_left as u16,
                    (bottom_left + 1) as u16,
                ]);
            }
        }

        let disk = Mesh {
            vertices: vec![Vertex::new2(Vec3::ZERO, Vec2::ZERO, WHITE,); vertex_count],
            indices,
            texture: None,
        };

        let mut rng = Rng::new(0xB1AC_401E);

        let particles = (0..1_800)
            .map(|_| {
                let t = rng.next_f32().powf(1.4);

                Particle {
                    radius: INNER_RADIUS + t * (OUTER_RADIUS - INNER_RADIUS),
                    phase: rng.range(0.0, TAU),
                    height: rng.range(-0.11, 0.11),
                    brightness: rng.range(0.4, 1.0),
                    length: rng.range(0.015, 0.085),
                }
            })
            .collect();

        Self { disk, particles }
    }

    pub fn draw(&mut self, time: f32, camera_position: Vec3, density: f32, show_jets: bool) {
        self.update_disk(time, camera_position);
        draw_mesh(&self.disk);

        self.draw_particles(time, density);

        draw_sphere(Vec3::ZERO, 1.02, None, Color::new(0.003, 0.004, 0.010, 1.0));

        self.draw_photon_ring(camera_position);

        if show_jets {
            self.draw_jets(time);
        }
    }

    fn update_disk(&mut self, time: f32, camera_position: Vec3) {
        let camera_yaw = camera_position.z.atan2(camera_position.x);

        for radial in 0..=RADIAL_STEPS {
            let t = radial as f32 / RADIAL_STEPS as f32;

            let radius = INNER_RADIUS + t.powf(1.15) * (OUTER_RADIUS - INNER_RADIUS);

            let omega = 2.7 / radius.powf(1.5);

            let color_base = temperature_color(t);

            for angular in 0..=ANGULAR_STEPS {
                let fraction = angular as f32 / ANGULAR_STEPS as f32;

                let angle = fraction * TAU + time * omega;

                let ripple = (angle * 9.0 - radius * 8.0 + time * 0.9).sin();

                let y = 0.026 * ripple * (0.4 + t);

                let position = vec3(radius * angle.cos(), y, radius * angle.sin());

                let spiral = (angle * 7.0 + radius * 12.0 - time * 0.4).sin();

                let filaments = 0.60 + 0.40 * spiral.abs().powf(4.0);

                let doppler_hint = 0.80 + 0.20 * (angle - camera_yaw).sin();

                let edge_fade = (1.0 - t.powf(2.5)).max(0.02);

                let glow = filaments * doppler_hint;

                let color = Color::new(
                    (color_base.r * glow).min(1.0),
                    (color_base.g * glow).min(1.0),
                    (color_base.b * glow).min(1.0),
                    (0.23 + 0.57 * edge_fade).clamp(0.0, 1.0),
                );

                let index = radial * (ANGULAR_STEPS + 1) + angular;

                self.disk.vertices[index].position = position;
                self.disk.vertices[index].uv = vec2(t, fraction);
                self.disk.vertices[index].color = color.into();
            }
        }
    }

    fn draw_particles(&self, time: f32, density: f32) {
        let visible_count = (self.particles.len() as f32 * density.clamp(0.1, 1.0)) as usize;

        for particle in self.particles.iter().take(visible_count) {
            let omega = 2.7 / particle.radius.powf(1.5);

            let angle = particle.phase + time * omega;
            let tail_angle = angle - particle.length;
            let y = particle.height;

            let from = vec3(
                particle.radius * tail_angle.cos(),
                y,
                particle.radius * tail_angle.sin(),
            );

            let to = vec3(
                particle.radius * angle.cos(),
                y,
                particle.radius * angle.sin(),
            );

            let radial_t = (particle.radius - INNER_RADIUS) / (OUTER_RADIUS - INNER_RADIUS);

            let base = temperature_color(radial_t);

            let alpha = particle.brightness * (1.0 - 0.55 * radial_t);

            let color = Color::new(base.r, base.g, base.b, alpha);

            draw_line_3d(from, to, color);
        }
    }

    fn draw_photon_ring(&self, camera_position: Vec3) {
        let towards_center = (-camera_position).normalize();

        let right = towards_center.cross(Vec3::Y).normalize();

        let up = right.cross(towards_center).normalize();

        for layer in 0..3 {
            let radius = 1.08 + layer as f32 * 0.027;

            let intensity = [0.95, 0.40, 0.17][layer];

            let color = Color::new(1.0, 0.67, 0.28, intensity);

            for segment in 0..180 {
                let a = segment as f32 / 180.0 * TAU;
                let b = (segment + 1) as f32 / 180.0 * TAU;

                let p1 = (right * a.cos() + up * a.sin()) * radius;

                let p2 = (right * b.cos() + up * b.sin()) * radius;

                draw_line_3d(p1, p2, color);
            }
        }
    }

    fn draw_jets(&self, time: f32) {
        for sign in [-1.0_f32, 1.0_f32] {
            for strand in 0..8 {
                let phase = strand as f32 / 8.0 * TAU;

                for step in 0..36 {
                    let h1 = 1.2 + step as f32 * 0.18;
                    let h2 = h1 + 0.18;

                    let p1 = jet_point(sign, h1, phase, time);

                    let p2 = jet_point(sign, h2, phase, time);

                    let alpha = 0.28 * (1.0 - step as f32 / 36.0);

                    draw_line_3d(p1, p2, Color::new(0.28, 0.64, 1.0, alpha));
                }
            }
        }
    }
}

fn jet_point(sign: f32, height: f32, phase: f32, time: f32) -> Vec3 {
    let angle = phase + time * 0.35 + height * 0.45;

    let width = 0.05 + 0.055 * height;

    vec3(width * angle.cos(), sign * height, width * angle.sin())
}

fn temperature_color(t: f32) -> Color {
    let stops = [
        (0.00, Color::new(1.00, 0.97, 0.81, 1.0)),
        (0.23, Color::new(1.00, 0.78, 0.32, 1.0)),
        (0.57, Color::new(1.00, 0.34, 0.08, 1.0)),
        (1.00, Color::new(0.45, 0.055, 0.08, 1.0)),
    ];

    for pair in stops.windows(2) {
        let (left_t, left) = pair[0];
        let (right_t, right) = pair[1];

        if t <= right_t {
            let mix = ((t - left_t) / (right_t - left_t)).clamp(0.0, 1.0);

            return Color::new(
                left.r + (right.r - left.r) * mix,
                left.g + (right.g - left.g) * mix,
                left.b + (right.b - left.b) * mix,
                1.0,
            );
        }
    }

    stops[stops.len() - 1].1
}
