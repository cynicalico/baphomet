mod buffer;
mod shader;
mod vec_buffer;

pub use buffer::*;
pub use shader::*;

// Restrict pub usage of VecBuffer to a few specializations
// It would likely blow up on much else than these
pub type FVecBuffer = vec_buffer::VecBuffer<f32>;
pub type IVecBuffer = vec_buffer::VecBuffer<i32>;
pub type UIVecBuffer = vec_buffer::VecBuffer<u32>;
