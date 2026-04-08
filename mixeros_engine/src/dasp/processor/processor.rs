use super::{buffer::ClBuffer, error::ClError};
use opencl3::{
    command_queue::{CL_QUEUE_PROFILING_ENABLE, CommandQueue},
    context::Context,
    device::{CL_DEVICE_TYPE_ACCELERATOR, CL_DEVICE_TYPE_ALL, CL_DEVICE_TYPE_CPU, CL_DEVICE_TYPE_GPU, CL_FP_DENORM, CL_FP_FMA, CL_FP_INF_NAN, CL_FP_ROUND_TO_NEAREST, Device},
    kernel::{ExecuteKernel, Kernel},
    platform::get_platforms,
};
use std::collections::HashMap;

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
    pub queue: CommandQueue,
    kernels: HashMap<String, Kernel>,
}

impl KernelManager {
    /// Create a manager
    /// Hiearchy for choosing a processor goes Requested -> GPU -> Accelerator -> CPU
    pub fn new(requested_device: Option<u64>) -> Result<Self, ClError> {
        let platforms = get_platforms().map_err(|e| ClError::InternalError(e))?;

        for (i, platform) in platforms.iter().enumerate() {
            println!("Platform {}: {}", i, platform.name().unwrap());
        }

        if platforms.is_empty() {
            eprintln!("Could not find any OpenCL platforms");
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
                        platforms.iter().enumerate().find_map(|(i, p)| {
                            p.get_devices(CL_DEVICE_TYPE_GPU)
                                .ok()
                                .and_then(|devs| devs.into_iter().next())
                                .map(|d| (Device::new(d), i))
                        })
                    })
                    .or_else(|| {
                        platforms.iter().enumerate().find_map(|(i, p)| {
                            p.get_devices(CL_DEVICE_TYPE_ACCELERATOR)
                                .ok()
                                .and_then(|devs| devs.into_iter().next())
                                .map(|d| (Device::new(d), i))
                        })
                    })
                    .or_else(|| {
                        platforms.iter().enumerate().find_map(|(i, p)| {
                            p.get_devices(CL_DEVICE_TYPE_CPU)
                                .ok()
                                .and_then(|devs| devs.into_iter().next())
                                .map(|d| (Device::new(d), i))
                        })
                    })
                    .ok_or(ClError::DeviceNotFound)?;
                device
            }
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
                        platforms.iter().enumerate().find_map(|(i, p)| {
                            p.get_devices(CL_DEVICE_TYPE_ACCELERATOR)
                                .ok()
                                .and_then(|devs| devs.into_iter().next())
                                .map(|d| (Device::new(d), i))
                        })
                    })
                    .or_else(|| {
                        platforms.iter().enumerate().find_map(|(i, p)| {
                            p.get_devices(CL_DEVICE_TYPE_CPU)
                                .ok()
                                .and_then(|devs| devs.into_iter().next())
                                .map(|d| (Device::new(d), i))
                        })
                    })
                    .or_else(|| {
                        platforms.iter().enumerate().find_map(|(i, p)| {
                            p.get_devices(CL_DEVICE_TYPE_ALL)
                                .ok()
                                .and_then(|devs| devs.into_iter().next())
                                .map(|d| (Device::new(d), i))
                        })
                    })
                    .ok_or(ClError::DeviceNotFound)?;

                device
            }
        };

        let fp_config = active_device.single_fp_config().expect("Failed to get fp config");
        let has_denorms = (fp_config & CL_FP_DENORM) != 0;
        let has_fma = (fp_config & CL_FP_FMA) != 0;
        let has_round_nearest = (fp_config & CL_FP_ROUND_TO_NEAREST) != 0;

        if has_denorms && has_fma && has_round_nearest == false {
            eprintln!("Found a device but it doesnt support all needed features: \n FMA: {} \n Denormal: {} Round to nearest: {}",
                has_fma,
                has_denorms,
                has_round_nearest
            );
            return Err(ClError::DeviceNotFound)
        }

        let ver_string = active_device.version().unwrap();

        // TODO Add support for multiple device (paralell processing over multiple devices)
        let context = Context::from_device(&active_device)
            .map_err(|e| ClError::OpenClError(format!("Error while creating context: {:?}", e)))?;

        let queue: CommandQueue;
        if ver_string.contains("OpenCL 2") && ver_string.contains("OpenCL 3") {
            queue =
                CommandQueue::create_default_with_properties(&context, CL_QUEUE_PROFILING_ENABLE, 0)
                    .map_err(|e| ClError::OpenClError(format!("Error while making queue: {:?}", e)))?;
        } else {
            unsafe {
                queue = 
                    CommandQueue::create(&context, active_device.id(), CL_QUEUE_PROFILING_ENABLE)
                    .map_err(|e| ClError::OpenClError(format!("Error while making queue: {:?}", e)))?;
            }
        }



        Ok(Self {
            context,
            queue,
            kernels: HashMap::new(),
        })
    }

    /// Compile from a file path instead of a source string
    pub fn add_program(&mut self, path: &str, kernel_names: &[&str]) -> Result<(), ClError> {
        let source = std::fs::read_to_string(path)
            .map_err(|e| ClError::OpenClError(format!("Failed to read {} while adding program: {}", path, e)))?;
        self.add_program(&source, kernel_names)
    }

    /// Run a kernel by name.
    ///
    /// `args`         — kernel arguments in order, matching the __kernel signature
    /// `global_size`  — total number of work items (e.g. number of samples)
    /// `local_size`   — work group size (None = let OpenCL decide)
    pub fn dispatch(
        &self,
        kernel_name: &str,
        args: &[KernelArg],
        global_size: usize,
        local_size: Option<usize>,
    ) -> Result<(), ClError> {
        let kernel = self
            .kernels
            .get(kernel_name)
            .ok_or_else(|| ClError::KernelNotFound(kernel_name.to_string()))?;

        // Set arguments
        for (i, arg) in args.iter().enumerate() {
            match arg {
                KernelArg::Buffer(buf) => unsafe {
                    kernel
                        .set_arg(i as u32, &buf.inner)
                        .map_err(|e| ClError::OpenClError(format!("Error while setting buffer: {:?}", e)))?;
                },
                KernelArg::F32(val) => unsafe {
                    kernel
                        .set_arg(i as u32, val)
                        .map_err(|e| ClError::OpenClError(format!("Error while setting float32: {:?}", e)))?;
                },
                KernelArg::U32(val) => unsafe {
                    kernel
                        .set_arg(i as u32, val)
                        .map_err(|e| ClError::OpenClError(format!("Error while setting unsigned integer 32: {:?}", e)))?;
                },
                KernelArg::I32(val) => unsafe {
                    kernel
                        .set_arg(i as u32, val)
                        .map_err(|e| ClError::OpenClError(format!("Error while setting integer 32: {:?}", e)))?;
                },
            }
        }

        // Enqueue
        let _local = local_size.map(|s| [s]);
        unsafe {
            ExecuteKernel::new(kernel)
                .set_global_work_size(global_size)
                .set_local_work_size(local_size.unwrap_or(64))
                .enqueue_nd_range(&self.queue)
                .map_err(|e| ClError::OpenClError(format!("Error when trying to execute kernel: Name: {} Err: {:?}", kernel_name, e)))?;
        }

        // Wait for completion
        self.queue
            .finish()
            .map_err(|e| ClError::OpenClError(format!("Error while waiting for kernel to run: {:?}", e)))?;

        Ok(())
    }

    /// Upload a Vec<f32> into an existing buffer
    pub fn write_buffer(&self, buf: &mut ClBuffer, data: &[f32]) -> Result<(), ClError> {
        unsafe {
            self.queue
                .enqueue_write_buffer(
                    &mut buf.inner,
                    opencl3::command_queue::CL_BLOCKING,
                    0,
                    data,
                    &[],
                )
                .map_err(|e| ClError::OpenClError(format!("{:?}", e)))?;
        }
        Ok(())
    }

    /// Read a buffer back into a Vec<f32>
    pub fn read_buffer(&self, buf: &ClBuffer) -> Result<Vec<f32>, ClError> {
        let mut result = vec![0.0f32; buf.len];
        unsafe {
            self.queue
                .enqueue_read_buffer(
                    &buf.inner,
                    opencl3::command_queue::CL_BLOCKING,
                    0,
                    &mut result,
                    &[],
                )
                .map_err(|e| ClError::OpenClError(format!("{:?}", e)))?;
        }
        Ok(result)
    }
}

unsafe impl Send for KernelManager {}
unsafe impl Sync for KernelManager {}
