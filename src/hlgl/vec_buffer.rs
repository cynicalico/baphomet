use crate::hlgl::buffer::GlBuffer;
use gl::types::{GLintptr, GLsizeiptr, GLuint};

pub type FVecBuffer = VecBuffer<f32>;

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

impl<T: Copy> GlBuffer for VecBuffer<T> {
    fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.id);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        }
    }
}

impl<T: Copy> VecBuffer<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        let id = unsafe {
            let mut id: u32 = 0;
            gl::GenBuffers(1, &mut id);
            id
        };

        let mut data = Vec::with_capacity(capacity);
        unsafe {
            data.set_len(capacity);
        }

        Self {
            id,
            data,
            front: 0,
            back: 0,
            gl_bufsize: 0,
            gl_bufpos: 0,
        }
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

    pub fn add<const N: usize>(&mut self, data: [T; N]) {
        while self.back + data.len() > self.data.len() {
            if self.data.len() == 0 {
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
                self.bind();
                gl::BufferData(
                    gl::ARRAY_BUFFER,
                    (self.data.len() * size_of::<T>()) as GLsizeiptr,
                    self.data[..self.data.len()].as_ptr().cast(),
                    gl::STATIC_DRAW,
                );
                self.unbind();
            }
            self.gl_bufsize = self.data.len();
            self.gl_bufpos = self.back;
        } else if self.back > self.gl_bufpos {
            unsafe {
                self.bind();
                gl::BufferSubData(
                    gl::ARRAY_BUFFER,
                    (self.gl_bufpos * size_of::<T>()) as GLintptr,
                    ((self.back - self.gl_bufpos) * size_of::<T>()) as GLsizeiptr,
                    self.data[self.gl_bufpos..self.back].as_ptr().cast(),
                );
                self.unbind();
            }
            self.gl_bufpos = self.back;
        }
    }
}
