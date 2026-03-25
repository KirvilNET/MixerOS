use opencl3::{
    command_queue::{CommandQueue, CL_QUEUE_PROFILING_ENABLE},
    context::Context,
    device::{Device, CL_DEVICE_TYPE_GPU, CL_DEVICE_TYPE_CPU},
    kernel::{Kernel, ExecuteKernel},
    platform::get_platforms,
};
use std::collections::HashMap;
use super::{
    buffer::{ ClBuffer },
    error::ClError,
};

/// A single kernel argument — either a buffer or a scalar push value
pub enum KernelArg<'a> {
    Buffer(&'a ClBuffer),
    F32(f32),
    U32(u32),
    I32(i32),
}

#[derive(Debug)]
pub struct KernelManager {
    pub context: Context,
    pub queue:   CommandQueue,
    kernels:     HashMap<String, Kernel>,
}

impl KernelManager {
    /// Create a manager, picking a GPU (falls back to CPU if no GPU is found)
    pub fn new(requested_device: Option<u64>) -> Result<Self, ClError> {
        let platforms = get_platforms()
            .map_err(|_| ClError::PlatformNotFound)?;

        if platforms.is_empty() {
            return Err(ClError::PlatformNotFound);
        }

        let active_device = match requested_device {
            Some(device) => {
              // Try each platform for the requested device type, fall back to GPU if it isnt, then fallback to CPU
              let (device, _platform_idx) = platforms
                  .iter()
                  .enumerate()
                  .find_map(|(i, p)| {
                      p.get_devices(device)
                          .ok()
                          .and_then(|devs| devs.into_iter().next())
                          .map(|d| (Device::new(d), i))
                  })
                  .or_else(|| {
                      platforms
                          .iter()
                          .enumerate()
                          .find_map(|(i, p)| {
                              p.get_devices(CL_DEVICE_TYPE_GPU)
                                  .ok()
                                  .and_then(|devs| devs.into_iter().next())
                                  .map(|d| (Device::new(d), i))
                          })
                  })
                  .or_else(|| {
                      platforms
                          .iter()
                          .enumerate()
                          .find_map(|(i, p)| {
                              p.get_devices(CL_DEVICE_TYPE_CPU)
                                  .ok()
                                  .and_then(|devs| devs.into_iter().next())
                                  .map(|d| (Device::new(d), i))
                          })
                  })
                  .ok_or(ClError::DeviceNotFound)?;
              
              device
            },
            None => {
              // Try each platform for a GPU, fall back to CPU
              let (device, _platform_idx) = platforms
                  .iter()
                  .enumerate()
                  .find_map(|(i, p)| {
                      p.get_devices(CL_DEVICE_TYPE_GPU)
                          .ok()
                          .and_then(|devs| devs.into_iter().next())
                          .map(|d| (Device::new(d), i))
                  })
                  .or_else(|| {
                      platforms
                          .iter()
                          .enumerate()
                          .find_map(|(i, p)| {
                              p.get_devices(CL_DEVICE_TYPE_CPU)
                                  .ok()
                                  .and_then(|devs| devs.into_iter().next())
                                  .map(|d| (Device::new(d), i))
                          })
                  })
                  .ok_or(ClError::DeviceNotFound)?;

              device
            },
        };

        let context = Context::from_device(&active_device)
            .map_err(|e| ClError::OpenClError(format!("{:?}", e)))?;

        let queue = CommandQueue::create_default_with_properties(
            &context,
            CL_QUEUE_PROFILING_ENABLE,
            0,
        ).map_err(|e| ClError::OpenClError(format!("{:?}", e)))?;

        Ok(Self {
            context,
            queue,
            kernels: HashMap::new(),
        })
    }

    /// Compile from a file path instead of a source string
    pub fn add_program(
        &mut self,
        path: &str,
        kernel_names: &[&str],
    ) -> Result<(), ClError> {
        let source = std::fs::read_to_string(path)
            .map_err(|e| ClError::OpenClError(format!("Failed to read {}: {}", path, e)))?;
        self.add_program(&source, kernel_names)
    }

    /// Run a kernel by name.
    ///
    /// `args`         — kernel arguments in order, matching the __kernel signature
    /// `global_size`  — total number of work items (e.g. number of samples)
    /// `local_size`   — work group size (None = let OpenCL decide)
    pub fn dispatch(&self, kernel_name: &str, args: &[KernelArg], global_size: usize, local_size: Option<usize>,) -> Result<(), ClError> {
      let kernel = self.kernels
        .get(kernel_name)
        .ok_or_else(|| ClError::KernelNotFound(kernel_name.to_string()))?;

      // Set arguments
      for (i, arg) in args.iter().enumerate() {
        match arg {
          KernelArg::Buffer(buf) => {
            unsafe {
              kernel.set_arg(i as u32, &buf.inner)
                .map_err(|e| ClError::OpenClError(format!("{:?}", e)))?;
            }
          }
          KernelArg::F32(val) => {
            unsafe {
              kernel.set_arg(i as u32, val)
                .map_err(|e| ClError::OpenClError(format!("{:?}", e)))?;
            }
          }
          KernelArg::U32(val) => {
            unsafe {
              kernel.set_arg(i as u32, val)
                .map_err(|e| ClError::OpenClError(format!("{:?}", e)))?;
            }
          }
          KernelArg::I32(val) => {
            unsafe {
              kernel.set_arg(i as u32, val)
                .map_err(|e| ClError::OpenClError(format!("{:?}", e)))?;
            }
          }
        }
      }

      // Enqueue
      let _local = local_size.map(|s| [s]);
      unsafe {
          ExecuteKernel::new(kernel)
              .set_global_work_size(global_size)
              .set_local_work_size(local_size.unwrap_or(64))
              .enqueue_nd_range(&self.queue)
              .map_err(|e| ClError::OpenClError(format!("{:?}", e)))?;
      }

      // Wait for completion
      self.queue.finish()
          .map_err(|e| ClError::OpenClError(format!("{:?}", e)))?;

      Ok(())
    }

    /// Upload a Vec<f32> into an existing buffer
    pub fn write_buffer(&self, buf: &mut ClBuffer, data: &[f32]) -> Result<(), ClError> {
        unsafe {
            self.queue.enqueue_write_buffer(&mut buf.inner, opencl3::command_queue::CL_BLOCKING, 0, data, &[])
                .map_err(|e| ClError::OpenClError(format!("{:?}", e)))?;
        }
        Ok(())
    }

    /// Read a buffer back into a Vec<f32>
    pub fn read_buffer(&self, buf: &ClBuffer) -> Result<Vec<f32>, ClError> {
        let mut result = vec![0.0f32; buf.len];
        unsafe {
            self.queue.enqueue_read_buffer(&buf.inner, opencl3::command_queue::CL_BLOCKING, 0, &mut result, &[])
                .map_err(|e| ClError::OpenClError(format!("{:?}", e)))?;
        }
        Ok(result)
    }
}

unsafe impl Send for KernelManager {}
unsafe impl Sync for KernelManager {}