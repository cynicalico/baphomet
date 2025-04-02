use crate::hlgl::{BindTarget, GlBuffer, Shader};
use gl::types::{GLenum, GLint, GLsizei, GLuint};
use regex::Regex;
use std::ptr;

pub struct VertexArray {
    pub id: GLuint,
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            log::trace!("Deleting VertexArray with id: {}", self.id);
            gl::DeleteVertexArrays(1, &self.id);
        }
    }
}

impl VertexArray {
    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }

    pub fn draw_arrays(&self, mode: GLenum, first: GLint, count: GLsizei) {
        unsafe {
            gl::DrawArrays(mode, first, count);
        }
    }

    pub fn draw_elements(&self, mode: GLenum, count: GLsizei) {
        unsafe {
            gl::DrawElements(mode, count, gl::UNSIGNED_INT, ptr::null());
        }
    }
}

pub struct VertexArrayBuilder {
    id: GLuint,
}

impl Default for VertexArrayBuilder {
    fn default() -> Self {
        VertexArrayBuilder::new()
    }
}

impl VertexArrayBuilder {
    pub fn new() -> Self {
        let id = unsafe {
            let mut id: u32 = 0;
            gl::GenVertexArrays(1, &mut id);
            log::trace!("Generated VertexArray with id: {}", id);
            id
        };

        Self { id }
    }

    pub fn attrib_pointer<T: GlBuffer>(
        self,
        shader: &mut Shader,
        buffer: &T,
        target: BindTarget,
        format: &str,
    ) -> Self {
        let re = Regex::new(r"(\w+):(\d+)([fiu])").unwrap();

        let mut attribs: Vec<(GLuint, usize, GLenum, usize)> = vec![];
        for (_, [attrib_name, count, type_]) in re.captures_iter(format).map(|c| c.extract()) {
            let Some(loc) = shader.attrib_loc(attrib_name) else {
                continue; // shader will log unable to find attrib, just skip it
            };

            let count = count.parse().unwrap();

            let (type_, size) = match type_ {
                "f" => (gl::FLOAT, size_of::<f32>()),
                "i" => (gl::INT, size_of::<i32>()),
                "u" => (gl::UNSIGNED_INT, size_of::<u32>()),
                _ => unreachable!(),
            };

            attribs.push((loc, count, type_, size));
        }

        let stride: usize = attribs.iter().map(|(_, count, _, _)| count).sum();

        unsafe {
            gl::BindVertexArray(self.id);
            buffer.bind(target);
            let mut offset = 0;
            for (loc, count, type_, size) in attribs {
                log::trace!(
                    "VertexArray (id: {}) attrib_pointer {} {} {} {} {} {}",
                    self.id,
                    loc,
                    count,
                    type_,
                    false,
                    stride * size,
                    offset
                );

                gl::VertexAttribPointer(
                    loc,
                    count as GLint,
                    type_,
                    gl::FALSE,
                    (stride * size) as GLsizei,
                    offset as *const _,
                );
                gl::EnableVertexAttribArray(loc);

                offset += count * size;
            }
            buffer.unbind(target);
            gl::BindVertexArray(0);
        }

        self
    }

    pub fn with_index_buffer<T: GlBuffer>(self, buffer: &T) -> Self {
        unsafe {
            gl::BindVertexArray(self.id);
            buffer.bind(BindTarget::ElementArrayBuffer);
            gl::BindVertexArray(0);
            buffer.unbind(BindTarget::ElementArrayBuffer);
        }

        self
    }

    pub fn build(self) -> VertexArray {
        VertexArray { id: self.id }
    }
}
