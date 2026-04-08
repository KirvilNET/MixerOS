use std::fmt;

#[derive(Debug)]
pub enum ClError {
    PlatformNotFound,
    DeviceNotFound,
    KernelNotFound(String),
    BuildFailed(String),
    OpenClError(String),
    InternalError(opencl3::error_codes::ClError),
}

impl fmt::Display for ClError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClError::PlatformNotFound => write!(f, "No OpenCL platform found"),
            ClError::DeviceNotFound => write!(f, "No OpenCL device found"),
            ClError::KernelNotFound(name) => write!(f, "Kernel '{}' not found", name),
            ClError::BuildFailed(log) => write!(f, "Build failed:\n{}", log),
            ClError::OpenClError(msg) => write!(f, "OpenCL error: {}", msg),
            ClError::InternalError(err) => write!(f, "Internal Error: {}", err)
        }
    }
}

impl From<opencl3::types::cl_int> for ClError {
    fn from(e: opencl3::types::cl_int) -> Self {
        ClError::OpenClError(format!("Error code: {}", e))
    }
}