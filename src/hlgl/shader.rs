use gl::types::{GLint, GLsizei, GLuint};
use pastey::paste;
use std::collections::HashMap;
use std::ffi::CString;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const INFO_LOG_MAX_LEN: usize = 1024;

#[derive(Debug)]
pub struct Shader {
    pub id: GLuint,
    uniform_locs: HashMap<String, Option<GLint>>,
}

macro_rules! uniform_vec {
    ($type_:tt, $param_type:ty, $($es:ident),+) => {
        paste! {
            pub fn [<uniform_ $type_>](&mut self, name: &str, $($es: $param_type),+) {
                if let Some(loc) = self.get_uniform_loc(name) {
                    unsafe {
                        gl::[<Uniform $type_>](*loc, $($es),+);
                    }
                }
            }
        }
    };

    ($type_:tt, $glm_type:ty) => {
        paste! {
            pub fn [<uniform_ $type_>](&mut self, name: &str, value: &$glm_type) {
                if let Some(loc) = self.get_uniform_loc(name) {
                    unsafe {
                        gl::[<Uniform $type_>](*loc, 1, value.as_ptr().cast());
                    }
                }
            }
        }
    };
}

macro_rules! uniform_mat {
    ($type_:tt, $glm_type:ty) => {
        paste! {
            pub fn [<uniform_mat $type_>](&mut self, name: &str, transpose: bool, value: &$glm_type) {
                if let Some(loc) = self.get_uniform_loc(name) {
                    unsafe {
                        gl::[<Uniform Matrix $type_>](*loc, 1, transpose.into(), value.as_ptr().cast());
                    }
                }
            }
        }
    };
}

impl Shader {
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    uniform_vec!(1f, f32, v0);
    uniform_vec!(2f, f32, v0, v1);
    uniform_vec!(3f, f32, v0, v1, v2);
    uniform_vec!(4f, f32, v0, v1, v2, v3);

    uniform_vec!(1i, i32, v0);
    uniform_vec!(2i, i32, v0, v1);
    uniform_vec!(3i, i32, v0, v1, v2);
    uniform_vec!(4i, i32, v0, v1, v2, v3);

    uniform_vec!(1ui, u32, v0);
    uniform_vec!(2ui, u32, v0, v1);
    uniform_vec!(3ui, u32, v0, v1, v2);
    uniform_vec!(4ui, u32, v0, v1, v2, v3);

    uniform_vec!(1fv, glm::Vec1);
    uniform_vec!(2fv, glm::Vec2);
    uniform_vec!(3fv, glm::Vec3);
    uniform_vec!(4fv, glm::Vec4);

    uniform_vec!(1iv, glm::IVec1);
    uniform_vec!(2iv, glm::IVec2);
    uniform_vec!(3iv, glm::IVec3);
    uniform_vec!(4iv, glm::IVec4);

    uniform_vec!(1uiv, glm::UVec1);
    uniform_vec!(2uiv, glm::UVec2);
    uniform_vec!(3uiv, glm::UVec3);
    uniform_vec!(4uiv, glm::UVec4);

    uniform_mat!(2fv, glm::Mat2);
    uniform_mat!(3fv, glm::Mat3);
    uniform_mat!(4fv, glm::Mat4);

    uniform_mat!(2x3fv, glm::Mat2x3);
    uniform_mat!(3x2fv, glm::Mat3x2);
    uniform_mat!(2x4fv, glm::Mat2x4);
    uniform_mat!(4x2fv, glm::Mat4x2);
    uniform_mat!(3x4fv, glm::Mat3x4);
    uniform_mat!(4x3fv, glm::Mat4x3);

    fn get_uniform_loc(&mut self, name: &str) -> &Option<GLint> {
        self.uniform_locs
            .entry(name.to_owned())
            .or_insert_with(|| unsafe {
                let loc = gl::GetUniformLocation(
                    self.id,
                    CString::from_str(name).unwrap().as_ptr().cast(),
                );
                if loc == -1 {
                    log::error!("Could not find uniform: '{}'", name);
                    None
                } else {
                    Some(loc)
                }
            })
    }
}

#[derive(Default)]
pub struct ShaderBuilder {
    shader_ids: Vec<GLuint>,
}

#[derive(Copy, Clone, Debug)]
pub enum ShaderKind {
    Vertex,
    Fragment,
}

impl Display for ShaderKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ShaderKind::Vertex => write!(f, "vertex"),
            ShaderKind::Fragment => write!(f, "fragment"),
        }
    }
}

impl ShaderBuilder {
    pub fn with_src(mut self, kind: ShaderKind, src: &str) -> Self {
        let id: u32 = unsafe {
            let gl_kind = match kind {
                ShaderKind::Vertex => gl::VERTEX_SHADER,
                ShaderKind::Fragment => gl::FRAGMENT_SHADER,
            };
            let id = gl::CreateShader(gl_kind);

            gl::ShaderSource(
                id,
                1,
                &src.as_bytes().as_ptr().cast(),
                &src.len().try_into().unwrap(),
            );
            gl::CompileShader(id);

            if Self::check_compile(id, kind) { id } else { 0 }
        };

        if id != 0 {
            self.shader_ids.push(id);
        }

        self
    }

    pub fn try_link(self) -> Option<Shader> {
        let id = unsafe {
            let id = gl::CreateProgram();

            for shader_id in &self.shader_ids {
                gl::AttachShader(id, *shader_id);
            }

            gl::LinkProgram(id);
            if Self::check_link(id) { id } else { 0 }
        };

        unsafe {
            for shader_id in self.shader_ids {
                gl::DeleteShader(shader_id);
            }
        }

        if id != 0 {
            Some(Shader {
                id,
                uniform_locs: Default::default(),
            })
        } else {
            None
        }
    }

    fn check_compile(id: GLuint, kind: ShaderKind) -> bool {
        unsafe {
            let mut success = 0;
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut info_log: Vec<u8> = Vec::with_capacity(INFO_LOG_MAX_LEN);
                let mut log_len = 0;
                gl::GetShaderInfoLog(
                    id,
                    info_log.capacity() as GLsizei,
                    &mut log_len,
                    info_log.as_mut_ptr().cast(),
                );
                info_log.set_len(log_len as usize);

                log::error!(
                    "Error(s) while compiling {} shader:\n{}",
                    kind,
                    String::from_utf8_lossy(&info_log).trim_end()
                );
            }

            success != 0
        }
    }

    fn check_link(id: GLuint) -> bool {
        unsafe {
            let mut success: GLint = 0;
            gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut info_log: Vec<u8> = Vec::with_capacity(INFO_LOG_MAX_LEN);
                let mut log_len: GLsizei = 0;
                gl::GetProgramInfoLog(
                    id,
                    info_log.capacity() as GLsizei,
                    &mut log_len,
                    info_log.as_mut_ptr().cast(),
                );
                info_log.set_len(log_len as usize);

                log::error!(
                    "Error(s) while linking shader program:\n{}",
                    String::from_utf8_lossy(&info_log).trim_end()
                );
            }

            success != 0
        }
    }
}
