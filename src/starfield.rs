use macroquad::prelude::*;

use crate::rng::Rng;

struct Star {
    x: f32,
    y: f32,
    radius: f32,
    brightness: f32,
    phase: f32,
}

pub struct Starfield {
    stars: Vec<Star>,
}

impl Starfield {
    pub fn new(count: usize) -> Self {
        let mut rng = Rng::new(0x57A4_1E5);

        let stars = (0..count)
            .map(|_| Star {
                x: rng.next_f32(),
                y: rng.next_f32(),
                radius: rng.range(0.5, 1.7),
                brightness: rng.range(0.28, 0.9),
                phase: rng.range(0.0, std::f32::consts::TAU),
            })
            .collect();
        Self { stars }
    }

    pub fn draw(&self, time: f32, camera_distance: f32, lensing: bool) {
        let center = vec2(screen_width() / 2.0, screen_height() / 2.0);

        let focal_length = screen_height() / (2.0 * (45.0_f32.to_radians() / 2.0).tan());

        let horizon_radius_px = focal_length / camera_distance;

        for star in &self.stars {
            let mut point = vec2(star.x * screen_width(), star.y * screen_height());

            if lensing {
                let offset = point - center;
                let radius = offset.length();

                if radius > 0.001 && radius < horizon_radius_px * 7.0 {
                    let displacement = (horizon_radius_px * horizon_radius_px * 0.65 / radius)
                        .min(horizon_radius_px * 0.75);

                    point += offset / radius * displacement;
                }
            }

            let twinkle = (time * 0.7 + star.phase).sin() * 0.12;

            let brightness = (star.brightness + twinkle).clamp(0.0, 1.0);

            draw_circle(
                point.x,
                point.y,
                star.radius,
                Color::new(0.60, 0.76, 1.0, brightness),
            );
        }
    }
}
