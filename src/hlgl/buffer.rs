use crate::{gl, gl::types::GLenum};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum BindTarget {
    ArrayBuffer,
    ElementArrayBuffer,
}

impl BindTarget {
    pub fn as_gl_enum(&self) -> GLenum {
        match self {
            BindTarget::ArrayBuffer => gl::ARRAY_BUFFER,
            BindTarget::ElementArrayBuffer => gl::ELEMENT_ARRAY_BUFFER,
        }
    }
}

pub trait GlBuffer {
    fn gen_id(&mut self);
    fn del_id(&mut self);
    fn bind(&self, target: BindTarget);
    fn unbind(&self, target: BindTarget);
}
