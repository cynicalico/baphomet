use crate::{
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

macro_rules! try_link_and_insert_shader {
    ($shaders:ident, $gl_version:ident, $kind:path, $kind_str:literal) => {
        let version_directive = format!("#version {}{}0 core\n", $gl_version.0, $gl_version.1);
        match ShaderBuilder::default()
            .with_src(
                ShaderKind::Vertex,
                &(version_directive.clone()
                    + include_str!(concat!("shader_src/", $kind_str, ".vert"))),
            )
            .with_src(
                ShaderKind::Fragment,
                &(version_directive + include_str!(concat!("shader_src/", $kind_str, ".frag"))),
            )
            .try_link()
        {
            Ok(shader) => {
                $shaders.insert($kind, shader);
            }
            Err(_) => {
                log::error!(concat!("Failed to link ", $kind_str, " shader"));
            }
        }
    };
}

impl Batcher {
    pub fn new(gl_version: (u8, u8)) -> Self {
        let mut shaders = HashMap::default();

        try_link_and_insert_shader!(shaders, gl_version, BatchKind::Points, "points");
        try_link_and_insert_shader!(shaders, gl_version, BatchKind::Lines, "lines");
        try_link_and_insert_shader!(shaders, gl_version, BatchKind::Tris, "tris");

        Self {
            batches: vec![],
            shaders,
        }
    }

    pub fn draw(&mut self, proj: &glm::Mat4, z_max: f32) {
        for batch in &mut self.batches {
            if let Some(shader) = self.shaders.get_mut(&batch.kind) {
                shader.use_program();
                shader.uniform_mat("proj", false, proj);
                shader.uniform_1("z_max", z_max);
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

    pub fn point(&mut self, z: f32, p: (f32, f32), color: (f32, f32, f32, f32)) {
        let batch = self.check_get_batch(BatchKind::Points);

        batch.vertices.add([p.0, p.1, z, color.0, color.1, color.2]);
    }

    pub fn line(&mut self, z: f32, p0: (f32, f32), p1: (f32, f32), color: (f32, f32, f32, f32)) {
        let batch = self.check_get_batch(BatchKind::Lines);

        #[rustfmt::skip]
        batch.vertices.add([
            p0.0, p0.1, z, color.0, color.1, color.2,
            p1.0, p1.1, z, color.0, color.1, color.2,
        ]);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn fill_tri(
        &mut self,
        z: f32,
        p0: (f32, f32),
        p1: (f32, f32),
        p2: (f32, f32),
        color: (f32, f32, f32, f32),
        p_rot: (f32, f32),
        angle: f32,
    ) {
        let batch = self.check_get_batch(BatchKind::Tris);

        let index_offset = (batch.vertices.size() / 9) as u32;
        batch
            .indices
            .as_mut()
            .unwrap()
            .add([index_offset, index_offset + 1, index_offset + 2]);

        #[rustfmt::skip]
        batch.vertices.add([
            p0.0, p0.1, z, color.0, color.1, color.2, p_rot.0, p_rot.1, angle,
            p1.0, p1.1, z, color.0, color.1, color.2, p_rot.0, p_rot.1, angle,
            p2.0, p2.1, z, color.0, color.1, color.2, p_rot.0, p_rot.1, angle,
        ]);
    }

    fn check_get_batch(&mut self, kind: BatchKind) -> &mut Batch {
        let needs_new_batch = match self.batches.last() {
            Some(batch) => batch.kind != kind,
            None => true,
        };

        if needs_new_batch {
            let new_batch = self.make_batch(kind);
            self.batches.push(new_batch);
        }

        self.batches.last_mut().unwrap()
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
