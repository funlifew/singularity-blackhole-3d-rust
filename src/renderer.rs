use macroquad::prelude::*;

use crate::{Settings, camera::OrbitCamera};

/// All shader assets are embedded: release binaries need no working-directory assets.
pub struct Renderer {
    rays: Material,
    post: Material,
    target: RenderTarget,
    size: (u32, u32),
}

impl Renderer {
    pub fn new() -> Result<Self, macroquad::Error> {
        let vertex = include_str!("shaders/fullscreen.vert");
        let rays = load_material(
            ShaderSource::Glsl {
                vertex,
                fragment: include_str!("shaders/blackhole.frag"),
            },
            MaterialParams {
                uniforms: vec![
                    UniformDesc::new("Resolution", UniformType::Float2),
                    UniformDesc::new("Eye", UniformType::Float3),
                    UniformDesc::new("Forward", UniformType::Float3),
                    UniformDesc::new("Right", UniformType::Float3),
                    UniformDesc::new("Up", UniformType::Float3),
                    UniformDesc::new("Time", UniformType::Float1),
                    UniformDesc::new("Lensing", UniformType::Float1),
                    UniformDesc::new("Jets", UniformType::Float1),
                    UniformDesc::new("Density", UniformType::Float1),
                    UniformDesc::new("Steps", UniformType::Float1),
                ],
                ..Default::default()
            },
        )?;
        let post = load_material(
            ShaderSource::Glsl {
                vertex,
                fragment: include_str!("shaders/post.frag"),
            },
            MaterialParams {
                uniforms: vec![
                    UniformDesc::new("Texel", UniformType::Float2),
                    UniformDesc::new("Exposure", UniformType::Float1),
                    UniformDesc::new("Bloom", UniformType::Float1),
                ],
                ..Default::default()
            },
        )?;
        Ok(Self {
            rays,
            post,
            target: render_target(1, 1),
            size: (1, 1),
        })
    }

    pub fn draw(&mut self, camera: &OrbitCamera, settings: &Settings, time: f32) {
        let scale = settings.quality.scale();
        let size = (
            (screen_width() * scale).round().max(1.0) as u32,
            (screen_height() * scale).round().max(1.0) as u32,
        );
        if size != self.size {
            self.target = render_target(size.0, size.1);
            self.target.texture.set_filter(FilterMode::Linear);
            self.size = size;
        }
        let (forward, right, up) = camera.basis();
        set_camera(&Camera2D {
            render_target: Some(self.target.clone()),
            ..Camera2D::from_display_rect(Rect::new(0.0, 0.0, size.0 as f32, size.1 as f32))
        });
        self.rays
            .set_uniform("Resolution", vec2(size.0 as f32, size.1 as f32));
        self.rays.set_uniform("Eye", camera.position());
        self.rays.set_uniform("Forward", forward);
        self.rays.set_uniform("Right", right);
        self.rays.set_uniform("Up", up);
        self.rays.set_uniform("Time", time);
        self.rays
            .set_uniform("Lensing", if settings.lensing { 1.0_f32 } else { 0.0 });
        self.rays
            .set_uniform("Jets", if settings.jets { 1.0_f32 } else { 0.0 });
        self.rays.set_uniform("Density", settings.density);
        self.rays.set_uniform("Steps", settings.quality.steps());
        gl_use_material(&self.rays);
        draw_rectangle(0.0, 0.0, size.0 as f32, size.1 as f32, WHITE);
        gl_use_default_material();
        set_default_camera();
        self.post
            .set_uniform("Texel", vec2(1.0 / size.0 as f32, 1.0 / size.1 as f32));
        self.post.set_uniform("Exposure", settings.exposure);
        self.post.set_uniform("Bloom", settings.bloom);
        gl_use_material(&self.post);
        draw_texture_ex(
            &self.target.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                flip_y: true,
                ..Default::default()
            },
        );
        gl_use_default_material();
    }
}
