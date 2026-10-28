mod camera;
mod gui;
mod rng;
mod scene;
mod starfield;

use macroquad::prelude::*;

use camera::OrbitCamera;
use scene::Scene;
use starfield::Starfield;

struct Settings {
    speed: f32,
    density: f32,
    lensing: bool,
    jets: bool,
    paused: bool,
    show_panel: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            speed: 1.0,
            density: 0.65,
            lensing: true,
            jets: false,
            paused: false,
            show_panel: true,
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Singularity | Black Hole Simulator".to_owned(),
        window_width: 1280,
        window_height: 760,
        window_resizable: true,
        high_dpi: false,
        sample_count: 1,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut camera = OrbitCamera::default();
    let mut scene = Scene::new();

    let stars = Starfield::new(600);

    let mut settings = Settings::default();

    let mut elapsed = 0.0_f32;
    let mut smoothed_fps = 60.0_f32;

    loop {
        // Keyboard controls
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        if is_key_pressed(KeyCode::Space) {
            settings.paused = !settings.paused;
        }

        if is_key_pressed(KeyCode::H) {
            settings.show_panel = !settings.show_panel;
        }

        if is_key_pressed(KeyCode::R) {
            camera = OrbitCamera::default();
            elapsed = 0.0;
        }

        // Update camera from mouse input
        camera.update(settings.show_panel);

        // Frame-independent simulation time
        let delta = get_frame_time().clamp(0.0001, 0.05);

        if !settings.paused {
            elapsed += delta * settings.speed;
        }

        smoothed_fps = smoothed_fps * 0.94 + (1.0 / delta) * 0.06;

        // Clear the previous frame
        clear_background(Color::new(0.006, 0.010, 0.027, 1.0));

        // Background stars
        stars.draw(elapsed, camera.distance, settings.lensing);

        // 3D rendering
        set_camera(&camera.as_camera());

        scene.draw(elapsed, camera.position(), settings.density, settings.jets);

        // Switch back to 2D UI rendering
        set_default_camera();

        gui::draw_hud(&settings, smoothed_fps, elapsed);

        if settings.show_panel {
            gui::draw_controls(&mut settings);
        }

        // Wait for the next frame
        next_frame().await;
    }
}
