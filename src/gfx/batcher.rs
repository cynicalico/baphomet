use crate::gfx::color::GlColor;
use crate::hlgl::{
    BindTarget, FVecBuffer, Shader, ShaderBuilder, ShaderKind, UIVecBuffer, VertexArray,
    VertexArrayBuilder,
};
use gl::types::{GLenum, GLsizei};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum BatchKind {
    Tris,
}

impl BatchKind {
    fn as_gl_draw_mode(&self) -> GLenum {
        match self {
            BatchKind::Tris => gl::TRIANGLES,
        }
    }
}

pub struct Batch {
    kind: BatchKind,
    vao: VertexArray,
    vertices: FVecBuffer,
    indices: UIVecBuffer,
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
            unsafe {
                batch.vertices.sync();
                batch.indices.sync();
            }

            if let Some(shader) = self.shaders.get_mut(&batch.kind) {
                shader.use_program();
                shader.uniform_mat("proj", false, proj);
            }

            batch.vao.bind();
            batch.vao.draw_elements(
                batch.kind.as_gl_draw_mode(),
                batch.indices.size() as GLsizei,
            );
            batch.vao.unbind();
        }
    }

    pub fn fill_tri<T: GlColor>(
        &mut self,
        x0: f32,
        y0: f32,
        x1: f32,
        y1: f32,
        x2: f32,
        y2: f32,
        color: &T,
        rot_x: f32,
        rot_y: f32,
        angle: f32,
    ) {
        let (r, g, b, _) = color.gl_color();

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
            .add([index_offset, index_offset + 1, index_offset + 2]);

        #[rustfmt::skip]
        batch.vertices.add([
            x0, y0, 0.0, r, g, b, rot_x, rot_y, angle,
            x1, y1, 0.0, r, g, b, rot_x, rot_y, angle,
            x2, y2, 0.0, r, g, b, rot_x, rot_y, angle,
        ]);
    }

    fn make_batch(&mut self, kind: BatchKind) -> Batch {
        match kind {
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
                    indices,
                }
            }
        }
    }
}
