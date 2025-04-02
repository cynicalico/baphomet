use crate::{
    gl,
    gl::types::{GLintptr, GLsizeiptr, GLuint},
    hlgl::{BindTarget, buffer::GlBuffer},
};

pub struct VecBuffer<T: Copy> {
    pub id: GLuint,

    data: Vec<T>,
    front: usize,
    back: usize,

    gl_bufsize: usize,
    gl_bufpos: usize,
}

impl<T: Copy> Default for VecBuffer<T> {
    fn default() -> Self {
        Self::with_capacity(0)
    }
}

impl<T: Copy> Drop for VecBuffer<T> {
    fn drop(&mut self) {
        self.del_id();
    }
}

impl<T: Copy> GlBuffer for VecBuffer<T> {
    fn gen_id(&mut self) {
        unsafe {
            gl::GenBuffers(1, &mut self.id);
            log::trace!("Generated buffer (VecBuffer) with id: {}", self.id);
        }
    }

    fn del_id(&mut self) {
        unsafe {
            log::trace!("Deleting buffer (VecBuffer) with id: {}", self.id);
            gl::DeleteBuffers(1, &self.id);
            self.id = 0;
        }
    }

    fn bind(&self, target: BindTarget) {
        unsafe {
            gl::BindBuffer(target.as_gl_enum(), self.id);
        }
    }

    fn unbind(&self, target: BindTarget) {
        unsafe {
            gl::BindBuffer(target.as_gl_enum(), 0);
        }
    }
}

impl<T: Copy> VecBuffer<T> {
    #[allow(clippy::uninit_vec)]
    pub fn with_capacity(capacity: usize) -> Self {
        let mut data = Vec::with_capacity(capacity);
        unsafe {
            data.set_len(capacity);
        }

        let mut vb = Self {
            id: 0,
            data,
            front: 0,
            back: 0,
            gl_bufsize: 0,
            gl_bufpos: 0,
        };
        vb.gen_id();

        vb
    }

    pub fn clear(&mut self) {
        self.front = 0;
        self.back = 0;
        self.gl_bufpos = 0;
    }

    pub fn front(&self) -> usize {
        self.front
    }

    pub fn back(&self) -> usize {
        self.back
    }

    pub fn size(&self) -> usize {
        self.back - self.front
    }

    #[allow(clippy::uninit_vec)]
    pub fn add<const N: usize>(&mut self, data: [T; N]) {
        while self.back + data.len() > self.data.len() {
            if self.data.is_empty() {
                self.data.reserve(1);
                unsafe {
                    self.data.set_len(1);
                }
            } else {
                self.data.reserve(self.data.len());
                unsafe {
                    self.data.set_len(self.data.len() * 2);
                }
            }
        }
        self.data[self.back..self.back + data.len()].copy_from_slice(&data);
        self.back += data.len();
    }

    pub unsafe fn sync(&mut self) {
        if self.data.len() > self.gl_bufsize {
            unsafe {
                self.bind(BindTarget::ArrayBuffer);
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (self.data.len() * size_of::<T>()) as GLsizeiptr,
                    self.data[..self.data.len()].as_ptr().cast(),
                    gl::DYNAMIC_DRAW,
                );
                self.unbind(BindTarget::ArrayBuffer);
            }

            let old_size = self.gl_bufsize;
            self.gl_bufsize = self.data.len();
            self.gl_bufpos = self.back;

            log::trace!(
                "VecBuffer (id: {}) resized GL buffer {} -> {}",
                self.id,
                old_size,
                self.gl_bufsize
            );
        } else if self.back > self.gl_bufpos {
            unsafe {
                self.bind(BindTarget::ArrayBuffer);
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    (self.gl_bufpos * size_of::<T>()) as GLintptr,
                    ((self.back - self.gl_bufpos) * size_of::<T>()) as GLsizeiptr,
                    self.data[self.gl_bufpos..self.back].as_ptr().cast(),
                );
                self.unbind(BindTarget::ArrayBuffer);
            }
            self.gl_bufpos = self.back;
        }
    }
}
