use opencl3::{
    context::Context,
    memory::{Buffer, CL_MEM_READ_ONLY, CL_MEM_WRITE_ONLY, CL_MEM_READ_WRITE, CL_MEM_COPY_HOST_PTR},
    types::cl_float,
};
use std::ptr;
use super::error::*;

pub enum BufferAccess {
    /// CPU writes, GPU reads
    ReadOnly,
    /// GPU writes, CPU reads
    WriteOnly,
    /// GPU reads and writes
    ReadWrite,
}

pub struct ClBuffer {
    pub inner: Buffer<cl_float>,
    pub len: usize,
}

impl ClBuffer {
    /// Allocate an empty buffer on the GPU
    pub fn new(context: &Context, len: usize, access: BufferAccess) -> Result<Self, ClError> {
        let flags = match access {
            BufferAccess::ReadOnly  => CL_MEM_READ_ONLY,
            BufferAccess::WriteOnly => CL_MEM_WRITE_ONLY,
            BufferAccess::ReadWrite => CL_MEM_READ_WRITE,
        };

        let inner = unsafe {
            Buffer::<cl_float>::create(context, flags, len, ptr::null_mut())
                .map_err(|e| ClError::OpenClError(format!("{:?}", e)))?
        };

        Ok(Self { inner, len })
    }

    /// Allocate and immediately upload data from a slice
    pub fn from_slice(context: &Context, data: &[f32], access: BufferAccess) -> Result<Self, ClError> {
        let flags = match access {
            BufferAccess::ReadOnly  => CL_MEM_READ_ONLY  | CL_MEM_COPY_HOST_PTR,
            BufferAccess::WriteOnly => CL_MEM_WRITE_ONLY | CL_MEM_COPY_HOST_PTR,
            BufferAccess::ReadWrite => CL_MEM_READ_WRITE | CL_MEM_COPY_HOST_PTR,
        };

        let inner = unsafe {
            Buffer::<cl_float>::create(context, flags, data.len(), data.as_ptr() as *mut _)
                .map_err(|e| ClError::OpenClError(format!("{:?}", e)))?
        };

        Ok(Self { inner, len: data.len() })
    }
}