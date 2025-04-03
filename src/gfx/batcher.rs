use crate::{
    gfx::color::GlColor,
    gl,
    gl::types::{GLenum, GLsizei},
    hlgl::{
        BindTarget, FVecBuffer, Shader, ShaderBuilder, ShaderKind, UIVecBuffer, VertexArray,
        VertexArrayBuilder,
    },
};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum BatchKind {
    Points,
    Lines,
    Tris,
}

impl BatchKind {
    fn as_gl_draw_mode(&self) -> GLenum {
        match self {
            BatchKind::Points => gl::POINTS,
            BatchKind::Lines => gl::LINES,
            BatchKind::Tris => gl::TRIANGLES,
        }
    }
}

pub struct Batch {
    kind: BatchKind,
    vao: VertexArray,
    vertices: FVecBuffer,
    indices: Option<UIVecBuffer>,
}

impl Batch {
    pub fn sync(&mut self) {
        unsafe {
            self.vertices.sync();
            if let Some(indices) = &mut self.indices {
                indices.sync();
            }
        }
    }
}

pub struct Batcher {
    batches: Vec<Batch>,
    shaders: HashMap<BatchKind, Shader>,
}

impl Default for Batcher {
    fn default() -> Self {
        Self::new()
    }
}

impl Batcher {
    pub fn new() -> Self {
        let mut shaders = HashMap::default();

        match ShaderBuilder::default()
            .with_src(ShaderKind::Vertex, include_str!("shader_src/points.vert"))
            .with_src(ShaderKind::Fragment, include_str!("shader_src/points.frag"))
            .try_link()
        {
            Ok(shader) => {
                shaders.insert(BatchKind::Points, shader);
            }
            Err(_) => {
                log::error!("Failed to link points shader");
            }
        }

        match ShaderBuilder::default()
            .with_src(ShaderKind::Vertex, include_str!("shader_src/lines.vert"))
            .with_src(ShaderKind::Fragment, include_str!("shader_src/lines.frag"))
            .try_link()
        {
            Ok(shader) => {
                shaders.insert(BatchKind::Lines, shader);
            }
            Err(_) => {
                log::error!("Failed to link lines shader");
            }
        }

        match ShaderBuilder::default()
            .with_src(ShaderKind::Vertex, include_str!("shader_src/tris.vert"))
            .with_src(ShaderKind::Fragment, include_str!("shader_src/tris.frag"))
            .try_link()
        {
            Ok(shader) => {
                shaders.insert(BatchKind::Tris, shader);
            }
            Err(_) => {
                log::error!("Failed to link tris shader");
            }
        }

        Self {
            batches: vec![],
            shaders,
        }
    }

    pub fn draw(&mut self, proj: &glm::Mat4) {
        for batch in &mut self.batches {
            if let Some(shader) = self.shaders.get_mut(&batch.kind) {
                shader.use_program();
                shader.uniform_mat("proj", false, proj);
            }

            batch.sync();
            batch.vao.bind();
            match batch.kind {
                BatchKind::Points | BatchKind::Lines => batch.vao.draw_arrays(
                    batch.kind.as_gl_draw_mode(),
                    0,
                    (batch.vertices.size() / 6) as GLsizei,
                ),
                BatchKind::Tris => {
                    batch.vao.draw_elements(
                        batch.kind.as_gl_draw_mode(),
                        batch.indices.as_ref().unwrap().size() as GLsizei,
                    );
                }
            }
            batch.vao.unbind();
        }
    }

    pub fn point<T: GlColor>(&mut self, p: (f32, f32), color: &T) {
        let needs_new_batch = match self.batches.last() {
            Some(batch) => batch.kind != BatchKind::Points,
            None => true,
        };

        if needs_new_batch {
            let new_batch = self.make_batch(BatchKind::Points);
            self.batches.push(new_batch);
        }

        let batch = self.batches.last_mut().unwrap();

        let (r, g, b, _) = color.gl_color();

        batch.vertices.add([p.0, p.1, 0.0, r, g, b]);
    }

    pub fn line<T: GlColor>(&mut self, p0: (f32, f32), p1: (f32, f32), color: &T) {
        let needs_new_batch = match self.batches.last() {
            Some(batch) => batch.kind != BatchKind::Lines,
            None => true,
        };

        if needs_new_batch {
            let new_batch = self.make_batch(BatchKind::Lines);
            self.batches.push(new_batch);
        }

        let batch = self.batches.last_mut().unwrap();

        let (r, g, b, _) = color.gl_color();

        #[rustfmt::skip]
        batch.vertices.add([
            p0.0, p0.1, 0.0, r, g, b,
            p1.0, p1.1, 0.0, r, g, b,
        ]);
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
        let needs_new_batch = match self.batches.last() {
            Some(batch) => batch.kind != BatchKind::Tris,
            None => true,
        };

        if needs_new_batch {
            let new_batch = self.make_batch(BatchKind::Tris);
            self.batches.push(new_batch);
        }

        let batch = self.batches.last_mut().unwrap();

        let index_offset = (batch.vertices.size() / 9) as u32;
        batch
            .indices
            .as_mut()
            .unwrap()
            .add([index_offset, index_offset + 1, index_offset + 2]);

        let (r, g, b, _) = color.gl_color();

        #[rustfmt::skip]
        batch.vertices.add([
            p0.0, p0.1, 0.0, r, g, b, p_rot.0, p_rot.1, angle,
            p1.0, p1.1, 0.0, r, g, b, p_rot.0, p_rot.1, angle,
            p2.0, p2.1, 0.0, r, g, b, p_rot.0, p_rot.1, angle,
        ]);
    }

    fn make_batch(&mut self, kind: BatchKind) -> Batch {
        match kind {
            BatchKind::Points | BatchKind::Lines => {
                let vertices = FVecBuffer::with_capacity(6 * 3);

                let vao = VertexArrayBuilder::default()
                    .attrib_pointer(
                        self.shaders.get_mut(&kind).unwrap(),
                        &vertices,
                        BindTarget::ArrayBuffer,
                        "pos:3f color:3f",
                    )
                    .build();

                Batch {
                    kind,
                    vao,
                    vertices,
                    indices: None,
                }
            }
            BatchKind::Tris => {
                let vertices = FVecBuffer::with_capacity(9 * 3);
                let indices = UIVecBuffer::with_capacity(3);

                let vao = VertexArrayBuilder::default()
                    .attrib_pointer(
                        self.shaders.get_mut(&kind).unwrap(),
                        &vertices,
                        BindTarget::ArrayBuffer,
                        "pos:3f color:3f rot_params:3f",
                    )
                    .with_index_buffer(&indices)
                    .build();

                Batch {
                    kind,
                    vao,
                    vertices,
                    indices: Some(indices),
                }
            }
        }
    }
}
