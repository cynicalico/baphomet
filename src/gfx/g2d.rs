use crate::gfx::GlColor;
use crate::gfx::batcher::Batcher;

pub struct G2d {
    batcher: Batcher,
    z_level: f32,
}

impl G2d {
    pub fn new(gl_version: (u8, u8)) -> Self {
        Self {
            batcher: Batcher::new(gl_version),
            z_level: 1.0,
        }
    }

    pub fn draw(&mut self, proj: &glm::Mat4) {
        self.batcher.draw(proj, self.z_level);
    }

    pub fn point<T: GlColor>(&mut self, p: (f32, f32), color: &T) {
        let gl_color = color.gl_color();
        self.batcher.point(self.z_level, p, gl_color);
        self.z_level += 1.0;
    }

    pub fn line<T: GlColor>(&mut self, p0: (f32, f32), p1: (f32, f32), color: &T) {
        let gl_color = color.gl_color();
        self.batcher.line(self.z_level, p0, p1, gl_color);
        self.z_level += 1.0;
    }

    pub fn fill_tri<T: GlColor>(
        &mut self,
        p0: (f32, f32),
        p1: (f32, f32),
        p2: (f32, f32),
        color: &T,
        p_rot: (f32, f32),
        angle: f32,
    ) {
        let gl_color = color.gl_color();
        self.batcher
            .fill_tri(self.z_level, p0, p1, p2, gl_color, p_rot, angle);
        self.z_level += 1.0;
    }
}
