use macroquad::prelude::*;

pub struct OrbitCamera {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    dragging: bool,
    previous_mouse: Vec2,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            yaw: 0.8,
            pitch: 0.36,
            distance: 13.8,
            dragging: false,
            previous_mouse: vec2(0.0, 0.0),
        }
    }
}

impl OrbitCamera {
    pub fn position(&self) -> Vec3 {
        vec3(
            self.distance * self.pitch.cos() * self.yaw.cos(),
            self.distance * self.pitch.sin(),
            self.distance * self.pitch.cos() * self.yaw.sin(),
        )
    }

    pub fn as_camera(&self) -> Camera3D {
        Camera3D {
            position: self.position(),
            target: Vec3::ZERO,
            up: Vec3::Y,
            fovy: 45.0_f32.to_radians(),
            z_near: 0.1,
            z_far: 150.0,
            ..Default::default()
        }
    }

    pub fn update(&mut self, panel_visible: bool) {
        let (x, y) = mouse_position();
        let mouse = vec2(x, y);

        let over_panel = panel_visible && x > screen_width() - 350.0 && y < 385.0;

        if is_mouse_button_pressed(MouseButton::Left) && !over_panel {
            self.dragging = true;
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.dragging = false;
        }

        if self.dragging && is_mouse_button_down(MouseButton::Left) {
            let delta = mouse - self.previous_mouse;

            self.yaw += delta.x * 0.006;
            self.pitch = (self.pitch + delta.y * 0.006).clamp(-1.40, 1.40);
        }

        if !over_panel {
            let (_, wheel_y) = mouse_wheel();

            self.distance = (self.distance - wheel_y * 0.85).clamp(7.0, 28.0);
        }

        self.previous_mouse = mouse;
    }
}
