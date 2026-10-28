use macroquad::prelude::*;

use crate::settings::Settings;

const TEXT: Color = Color::new(0.85, 0.88, 0.92, 1.0);
const MUTED: Color = Color::new(0.48, 0.54, 0.63, 1.0);
const ACCENT: Color = Color::new(1.0, 0.61, 0.29, 1.0);
const SURFACE: Color = Color::new(0.12, 0.15, 0.20, 1.0);

#[derive(Default)]
pub struct Panel {
    active_slider: Option<u8>,
}

pub fn panel_rect() -> Rect {
    Rect::new((screen_width() - 308.0).max(8.0), 24.0, 284.0, 365.0)
}

pub fn draw_hud(settings: &Settings, fps: f32, elapsed: f32) {
    draw_rectangle(
        24.0,
        24.0,
        270.0,
        98.0,
        Color::new(0.02, 0.025, 0.035, 0.82),
    );
    draw_rectangle(24.0, 24.0, 2.0, 98.0, ACCENT);
    draw_text("S I N G U L A R I T Y", 42.0, 58.0, 23.0, TEXT);
    draw_text("A BLACK HOLE EXPLORER", 42.0, 80.0, 15.0, MUTED);
    draw_text("RUST / GPU RAY TRACING", 42.0, 104.0, 15.0, ACCENT);
    if screen_height() > 460.0 {
        let y = screen_height() - 65.0;
        draw_rectangle(
            24.0,
            y - 24.0,
            screen_width().min(730.0) - 48.0,
            65.0,
            Color::new(0.02, 0.025, 0.035, 0.8),
        );
        draw_text(
            "DRAG  Orbit   SCROLL  Zoom   SPACE  Pause   H  Hide UI",
            40.0,
            y,
            16.0,
            TEXT,
        );
        draw_text(
            "L  Lens   J  Jets   O  Auto orbit   Q  Quality   R  Reset   F12  Capture",
            40.0,
            y + 23.0,
            15.0,
            MUTED,
        );
        draw_text(
            format!(
                "{}  /  {}  /  {:.0} FPS  /  T+{:.1}",
                if settings.paused { "PAUSED" } else { "LIVE" },
                settings.quality.label(),
                fps,
                elapsed
            ),
            40.0,
            y - 42.0,
            15.0,
            ACCENT,
        );
    }
}

pub fn draw_controls(settings: &mut Settings, panel: &mut Panel) {
    if !is_mouse_button_down(MouseButton::Left) {
        panel.active_slider = None;
    }
    if screen_width() < 680.0 || screen_height() < 440.0 {
        return;
    }
    let r = panel_rect();
    draw_rectangle(r.x, r.y, r.w, r.h, Color::new(0.025, 0.035, 0.05, 0.94));
    draw_rectangle_lines(r.x, r.y, r.w, r.h, 1.0, SURFACE);
    draw_text("O B S E R V A T O R Y", r.x + 18.0, r.y + 29.0, 18.0, TEXT);
    draw_text("LIGHT & MATTER", r.x + 18.0, r.y + 51.0, 13.0, ACCENT);
    slider(
        panel,
        0,
        r,
        76.0,
        "Exposure",
        &mut settings.exposure,
        (0.3, 2.5),
    );
    slider(
        panel,
        1,
        r,
        106.0,
        "Bloom",
        &mut settings.bloom,
        (0.0, 0.65),
    );
    slider(
        panel,
        2,
        r,
        136.0,
        "Opacity",
        &mut settings.density,
        (0.1, 1.0),
    );
    slider(
        panel,
        3,
        r,
        166.0,
        "Time scale",
        &mut settings.speed,
        (0.0, 4.0),
    );
    draw_line(
        r.x + 18.0,
        r.y + 187.0,
        r.right() - 18.0,
        r.y + 187.0,
        1.0,
        SURFACE,
    );
    checkbox(r, 200.0, "Gravitational lensing", &mut settings.lensing);
    checkbox(r, 227.0, "Polar jets", &mut settings.jets);
    checkbox(r, 254.0, "Auto orbit", &mut settings.auto_orbit);
    checkbox(r, 281.0, "Pause time", &mut settings.paused);
    if button(
        Rect::new(r.x + 16.0, r.y + 312.0, 121.0, 31.0),
        &format!("{} [Q]", settings.quality.label()),
    ) {
        settings.quality = settings.quality.next();
    }
    if button(
        Rect::new(r.x + 147.0, r.y + 312.0, 121.0, 31.0),
        "Reset visuals",
    ) {
        *settings = Settings::default();
    }
}

fn mouse() -> Vec2 {
    let (x, y) = mouse_position();
    vec2(x, y)
}

fn slider(
    panel: &mut Panel,
    id: u8,
    panel_rect: Rect,
    y: f32,
    label: &str,
    value: &mut f32,
    range: (f32, f32),
) {
    let x = panel_rect.x + 105.0;
    let y = panel_rect.y + y;
    let hit = Rect::new(x - 5.0, y - 9.0, 115.0, 20.0);
    if is_mouse_button_pressed(MouseButton::Left) && hit.contains(mouse()) {
        panel.active_slider = Some(id);
    }
    if panel.active_slider == Some(id) {
        let t = ((mouse().x - x) / 105.0).clamp(0.0, 1.0);
        *value = range.0 + (range.1 - range.0) * t;
    }
    let t = ((*value - range.0) / (range.1 - range.0)).clamp(0.0, 1.0);
    draw_text(label, panel_rect.x + 18.0, y + 5.0, 15.0, MUTED);
    draw_rectangle(x, y - 2.0, 105.0, 3.0, SURFACE);
    draw_rectangle(x, y - 2.0, 105.0 * t, 3.0, ACCENT);
    draw_circle(x + t * 105.0, y, 4.0, TEXT);
    draw_text(
        format!("{value:.2}"),
        panel_rect.right() - 56.0,
        y + 5.0,
        14.0,
        TEXT,
    );
}

fn checkbox(panel_rect: Rect, y: f32, label: &str, value: &mut bool) {
    let rect = Rect::new(
        panel_rect.x + 18.0,
        panel_rect.y + y,
        panel_rect.w - 36.0,
        22.0,
    );
    let hovered = rect.contains(mouse());
    if hovered && is_mouse_button_pressed(MouseButton::Left) {
        *value = !*value;
    }
    draw_rectangle(
        rect.x,
        rect.y + 3.0,
        14.0,
        14.0,
        if *value { ACCENT } else { SURFACE },
    );
    if *value {
        draw_line(
            rect.x + 3.0,
            rect.y + 10.0,
            rect.x + 6.0,
            rect.y + 13.0,
            2.0,
            BLACK,
        );
        draw_line(
            rect.x + 6.0,
            rect.y + 13.0,
            rect.x + 11.0,
            rect.y + 7.0,
            2.0,
            BLACK,
        );
    }
    draw_text(
        label,
        rect.x + 26.0,
        rect.y + 15.0,
        15.0,
        if hovered { WHITE } else { TEXT },
    );
}

fn button(rect: Rect, text: &str) -> bool {
    let hovered = rect.contains(mouse());
    draw_rectangle(
        rect.x,
        rect.y,
        rect.w,
        rect.h,
        if hovered {
            Color::new(0.22, 0.27, 0.34, 1.0)
        } else {
            SURFACE
        },
    );
    let width = measure_text(text, None, 14, 1.0).width;
    draw_text(
        text,
        rect.x + (rect.w - width) * 0.5,
        rect.y + 20.0,
        14.0,
        TEXT,
    );
    hovered && is_mouse_button_pressed(MouseButton::Left)
}
