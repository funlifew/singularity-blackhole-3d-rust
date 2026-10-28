mod camera;
mod gui;
mod options;
mod renderer;
mod settings;

use macroquad::prelude::*;
use options::{Options, Preset};
use renderer::Renderer;
use settings::Settings;

fn window_conf() -> Conf {
    let options = Options::from_env();
    Conf {
        window_title: "Singularity | Black Hole Explorer".into(),
        window_width: options.width,
        window_height: options.height,
        window_resizable: true,
        high_dpi: false,
        sample_count: 1,
        platform: macroquad::miniquad::conf::Platform {
            apple_gfx_api: macroquad::miniquad::conf::AppleGfxApi::OpenGl,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let options = Options::from_env();
    let mut camera = options.preset.camera();
    let mut settings = Settings {
        quality: options.quality,
        show_ui: !options.no_ui,
        jets: options.preset == Preset::Jets,
        lensing: !options.no_lensing,
        ..Default::default()
    };
    let mut renderer = match Renderer::new() {
        Ok(renderer) => renderer,
        Err(error) => {
            eprintln!("Cannot initialize the renderer: {error}");
            #[cfg(not(target_arch = "wasm32"))]
            std::process::exit(1);
            #[cfg(target_arch = "wasm32")]
            return;
        }
    };
    let mut panel_state = gui::Panel::default();
    let mut elapsed = options.time;
    let mut smoothed_fps = 0.0_f32;
    let mut first_frame = true;
    #[cfg(not(target_arch = "wasm32"))]
    let mut frame = 0;
    loop {
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::Space) {
            settings.paused = !settings.paused;
        }
        if is_key_pressed(KeyCode::H) {
            settings.show_ui = !settings.show_ui;
        }
        if is_key_pressed(KeyCode::L) {
            settings.lensing = !settings.lensing;
        }
        if is_key_pressed(KeyCode::J) {
            settings.jets = !settings.jets;
        }
        if is_key_pressed(KeyCode::O) {
            settings.auto_orbit = !settings.auto_orbit;
        }
        if is_key_pressed(KeyCode::Q) {
            settings.quality = settings.quality.next();
        }
        if is_key_pressed(KeyCode::R) {
            camera = options.preset.camera();
            elapsed = options.time;
        }
        let delta = get_frame_time().clamp(0.0001, 0.05);
        let panel = settings.show_ui.then(gui::panel_rect);
        camera.update(panel, settings.auto_orbit, delta);
        // Capture mode freezes simulation time for reproducible documentation and CI.
        if !settings.paused && options.capture.is_none() {
            elapsed += delta * settings.speed;
        }
        if !first_frame {
            let measured_fps = 1.0 / get_frame_time().max(0.0001);
            smoothed_fps = if smoothed_fps == 0.0 {
                measured_fps
            } else {
                smoothed_fps + (measured_fps - smoothed_fps) * 0.06
            };
        }
        first_frame = false;
        clear_background(BLACK);
        renderer.draw(&camera, &settings, elapsed);
        if settings.show_ui {
            gui::draw_hud(&settings, smoothed_fps, elapsed);
            gui::draw_controls(&mut settings, &mut panel_state);
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            frame += 1;
            if let Some(path) = &options.capture {
                if frame >= options.frames {
                    save_capture(path);
                    break;
                }
            } else if is_key_pressed(KeyCode::F12) {
                let stamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis();
                save_capture(&format!("screenshots/singularity-{stamp}.png"));
            }
        }
        next_frame().await;
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_capture(path: &str) {
    let destination = std::path::Path::new(path);
    if let Some(parent) = destination.parent().filter(|p| !p.as_os_str().is_empty())
        && let Err(error) = std::fs::create_dir_all(parent)
    {
        eprintln!("Cannot create screenshot directory: {error}");
        std::process::exit(1);
    }
    // Macroquad flushes queued GPU commands before reading the framebuffer.
    get_screen_data().export_png(path);
    println!("Saved {path}");
}
