use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

use crate::Settings;

pub fn draw_hud(settings: &Settings, fps: f32, elapsed: f32) {
    let foreground = Color::new(0.84, 0.91, 1.0, 1.0);
    let muted = Color::new(0.48, 0.59, 0.72, 1.0);
    let accent = Color::new(0.25, 0.75, 1.0, 1.0);

    draw_rectangle(
        24.0,
        24.0,
        294.0,
        120.0,
        Color::new(0.018, 0.033, 0.065, 0.86),
    );
    draw_rectangle(24.0, 24.0, 4.0, 120.0, accent);

    draw_text("S I N G U L A R I T Y", 43.0, 65.0, 22.0, foreground);

    draw_text("INTERACTIVE 3D BLACK HOLE", 43.0, 87.0, 16.0, muted);

    draw_text("RUST   /   MACROQUAD", 43.0, 117.0, 17.0, accent);

    let bottom_y = screen_height() - 94.0;

    draw_rectangle(
        24.0,
        bottom_y,
        548.0,
        69.0,
        Color::new(0.018, 0.033, 0.065, 0.82),
    );

    draw_text(
        "DRAG   Orbit Camera        SCROLL  Zoom",
        39.0,
        bottom_y + 26.0,
        18.0,
        foreground,
    );

    draw_text(
        "SPACE  Pause       R   Reset       H   Controls        ESC    Exit",
        39.0,
        bottom_y + 53.0,
        17.0,
        muted,
    );

    let status = if settings.paused { "PAUSED" } else { "LIVE" };

    let status_color = if settings.paused { ORANGE } else { GREEN };

    draw_text(status, 42.0, screen_height() - 121.0, 17.0, status_color);

    draw_text(
        &format!("{fps:4.0} FPS | T+{elapsed:06.1}"),
        130.0,
        screen_height() - 121.0,
        17.0,
        muted,
    );
}

pub fn draw_controls(settings: &mut Settings) {
    let panel_x = (screen_width() - 343.0).max(10.0);

    widgets::Window::new(hash!(), vec2(panel_x, 28.0), vec2(325.0, 330.0))
        .label("SIMULATION CONTROLS")
        .movable(false)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, "Time and Orbital Dynamics");
            ui.slider(hash!(), "Orbit Speed", 0.0..4.0, &mut settings.speed);

            ui.slider(hash!(), "Particles", 0.1..1.0, &mut settings.density);

            ui.separator();
            ui.label(None, "Visual Effects");

            ui.checkbox(hash!(), "Gravitational Lens Effect", &mut settings.lensing);

            ui.checkbox(hash!(), "Polar Jets", &mut settings.jets);

            ui.separator();

            let button_label = if settings.paused {
                "Resume Time"
            } else {
                "Pause Time"
            };

            if ui.button(None, button_label) {
                settings.paused = !settings.paused;
            }

            if ui.button(None, "Reset Settings") {
                *settings = Settings::default();
            }

            ui.label(None, "Hint: H  Hides this panel");
        });
}
