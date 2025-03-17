use gl::types::{GLint, GLsizei, GLuint};
use glm::Scalar;
use pastey::paste;
use std::collections::HashMap;
use std::ffi::CString;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

const INFO_LOG_MAX_LEN: usize = 1024;

macro_rules! uniform_vec_fn {
    ($type_:tt, $glm_type:ty) => {
        paste! {
            fn [<uniform_ $type_>](&self, loc: GLint, value: &$glm_type) {
                unsafe {
                    gl::[<Uniform $type_>](loc, 1, value.as_ptr().cast());
                }
            }
        }
    };
}

macro_rules! uniform_vec_impl {
    ($type_:tt, $glm_type:ty) => {
        paste! {
            impl UniformVec for $glm_type {
                fn set(&self, shader: &Shader, loc: GLint) {
                    shader.[<uniform_ $type_>](loc, self);
                }
            }
        }
    };
}

macro_rules! uniform_mat_fn {
    ($type_:tt, $glm_type:ty) => {
        paste! {
            fn [<uniform_mat $type_>](&self, loc: GLint, transpose: bool, value: &$glm_type) {
                unsafe {
                    gl::[<Uniform Matrix $type_>](loc, 1, transpose.into(), value.as_ptr().cast());
                }
            }
        }
    };
}

macro_rules! uniform_mat_impl {
    ($type_:tt, $glm_type:ty) => {
        paste! {
            impl UniformMat for $glm_type {
                fn set(&self, shader: &Shader, loc: GLint, transpose: bool) {
                    shader.[<uniform_mat $type_>](loc, transpose, self);
                }
            }
        }
    };
}

pub trait UniformVec {
    fn set(&self, shader: &Shader, loc: GLint);
}

pub trait UniformMat {
    fn set(&self, shader: &Shader, loc: GLint, transpose: bool);
}

#[derive(Debug)]
pub struct Shader {
    pub id: GLuint,
    uniform_locs: HashMap<String, Option<GLint>>,
}

impl Shader {
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn uniform_1<T: Scalar>(&mut self, name: &str, v0: T)
    where
        glm::TVec1<T>: UniformVec,
    {
        self.uniform_vec(name, &glm::vec1(v0));
    }

    pub fn uniform_2<T: Scalar>(&mut self, name: &str, v0: T, v1: T)
    where
        glm::TVec2<T>: UniformVec,
    {
        self.uniform_vec(name, &glm::vec2(v0, v1));
    }

    pub fn uniform_3<T: Scalar>(&mut self, name: &str, v0: T, v1: T, v2: T)
    where
        glm::TVec3<T>: UniformVec,
    {
        self.uniform_vec(name, &glm::vec3(v0, v1, v2));
    }

    pub fn uniform_4<T: Scalar>(&mut self, name: &str, v0: T, v1: T, v2: T, v3: T)
    where
        glm::TVec4<T>: UniformVec,
    {
        self.uniform_vec(name, &glm::vec4(v0, v1, v2, v3));
    }

    pub fn uniform_vec<T: UniformVec>(&mut self, name: &str, value: &T) {
        if let Some(loc) = self.get_uniform_loc(name) {
            value.set(self, loc);
        }
    }

    pub fn uniform_mat<T: UniformMat>(&mut self, name: &str, transpose: bool, value: &T) {
        if let Some(loc) = self.get_uniform_loc(name) {
            value.set(self, loc, transpose);
        }
    }

    fn get_uniform_loc(&mut self, name: &str) -> Option<GLint> {
        *self
            .uniform_locs
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

    uniform_vec_fn!(1fv, glm::TVec1<f32>);
    uniform_vec_fn!(2fv, glm::TVec2<f32>);
    uniform_vec_fn!(3fv, glm::TVec3<f32>);
    uniform_vec_fn!(4fv, glm::TVec4<f32>);

    uniform_vec_fn!(1iv, glm::TVec1<i32>);
    uniform_vec_fn!(2iv, glm::TVec2<i32>);
    uniform_vec_fn!(3iv, glm::TVec3<i32>);
    uniform_vec_fn!(4iv, glm::TVec4<i32>);

    uniform_vec_fn!(1uiv, glm::TVec1<u32>);
    uniform_vec_fn!(2uiv, glm::TVec2<u32>);
    uniform_vec_fn!(3uiv, glm::TVec3<u32>);
    uniform_vec_fn!(4uiv, glm::TVec4<u32>);

    uniform_mat_fn!(2fv, glm::TMat2<f32>);
    uniform_mat_fn!(3fv, glm::TMat3<f32>);
    uniform_mat_fn!(4fv, glm::TMat4<f32>);

    uniform_mat_fn!(2x3fv, glm::TMat2x3<f32>);
    uniform_mat_fn!(3x2fv, glm::TMat3x2<f32>);
    uniform_mat_fn!(2x4fv, glm::TMat2x4<f32>);
    uniform_mat_fn!(4x2fv, glm::TMat4x2<f32>);
    uniform_mat_fn!(3x4fv, glm::TMat3x4<f32>);
    uniform_mat_fn!(4x3fv, glm::TMat4x3<f32>);
}

uniform_vec_impl!(1fv, glm::TVec1<f32>);
uniform_vec_impl!(2fv, glm::TVec2<f32>);
uniform_vec_impl!(3fv, glm::TVec3<f32>);
uniform_vec_impl!(4fv, glm::TVec4<f32>);

uniform_vec_impl!(1iv, glm::TVec1<i32>);
uniform_vec_impl!(2iv, glm::TVec2<i32>);
uniform_vec_impl!(3iv, glm::TVec3<i32>);
uniform_vec_impl!(4iv, glm::TVec4<i32>);

uniform_vec_impl!(1uiv, glm::TVec1<u32>);
uniform_vec_impl!(2uiv, glm::TVec2<u32>);
uniform_vec_impl!(3uiv, glm::TVec3<u32>);
uniform_vec_impl!(4uiv, glm::TVec4<u32>);

uniform_mat_impl!(2fv, glm::TMat2<f32>);
uniform_mat_impl!(3fv, glm::TMat3<f32>);
uniform_mat_impl!(4fv, glm::TMat4<f32>);

uniform_mat_impl!(2x3fv, glm::TMat2x3<f32>);
uniform_mat_impl!(3x2fv, glm::TMat3x2<f32>);
uniform_mat_impl!(2x4fv, glm::TMat2x4<f32>);
uniform_mat_impl!(4x2fv, glm::TMat4x2<f32>);
uniform_mat_impl!(3x4fv, glm::TMat3x4<f32>);
uniform_mat_impl!(4x3fv, glm::TMat4x3<f32>);

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
