use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct OrbitCamera {
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    dragging: bool,
    previous_mouse: Vec2,
    target_yaw: f32,
    target_pitch: f32,
    target_distance: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self::at(0.8, 0.22, 23.0)
    }
}

impl OrbitCamera {
    pub fn at(yaw: f32, pitch: f32, distance: f32) -> Self {
        let pitch = pitch.clamp(-1.45, 1.45);
        let distance = distance.clamp(12.0, 42.0);
        Self {
            yaw,
            pitch,
            distance,
            target_yaw: yaw,
            target_pitch: pitch,
            target_distance: distance,
            dragging: false,
            previous_mouse: Vec2::ZERO,
        }
    }

    pub fn position(&self) -> Vec3 {
        vec3(
            self.distance * self.pitch.cos() * self.yaw.cos(),
            self.distance * self.pitch.sin(),
            self.distance * self.pitch.cos() * self.yaw.sin(),
        )
    }

    pub fn basis(&self) -> (Vec3, Vec3, Vec3) {
        let forward = -self.position().normalize();
        let right = forward.cross(Vec3::Y).normalize();
        let up = right.cross(forward).normalize();
        (forward, right, up)
    }

    pub fn update(&mut self, panel: Option<Rect>, auto_orbit: bool, delta: f32) {
        let (x, y) = mouse_position();
        let mouse = vec2(x, y);
        let over_panel = panel.is_some_and(|rect| rect.contains(mouse));
        if is_mouse_button_pressed(MouseButton::Left) && !over_panel {
            self.dragging = true;
            self.previous_mouse = mouse;
        }
        if !is_mouse_button_down(MouseButton::Left) {
            self.dragging = false;
        }
        if self.dragging {
            let movement = mouse - self.previous_mouse;
            self.target_yaw -= movement.x * 0.005;
            self.target_pitch = (self.target_pitch + movement.y * 0.005).clamp(-1.45, 1.45);
        } else if auto_orbit {
            self.target_yaw += delta * 0.08;
        }
        if !over_panel {
            self.target_distance =
                (self.target_distance * (-mouse_wheel().1 * 0.07).exp()).clamp(12.0, 42.0);
        }
        let blend = 1.0 - (-12.0 * delta).exp();
        self.yaw += (self.target_yaw - self.yaw) * blend;
        self.pitch += (self.target_pitch - self.pitch) * blend;
        self.distance += (self.target_distance - self.distance) * blend;
        self.previous_mouse = mouse;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn camera_basis_is_orthonormal_at_orbit_limits() {
        for pitch in [-1.45, -0.22, 0.0, 0.22, 1.45] {
            let camera = OrbitCamera::at(0.8, pitch, 23.0);
            let (forward, right, up) = camera.basis();
            assert!((camera.position().length() - 23.0).abs() < 1e-5);
            for axis in [forward, right, up] {
                assert!((axis.length() - 1.0).abs() < 1e-5);
            }
            assert!(forward.dot(right).abs() < 1e-5);
            assert!(right.dot(up).abs() < 1e-5);
            assert!(up.dot(forward).abs() < 1e-5);
        }
    }
}
