use gpu_allocator::vulkan::Allocator;
use gpu_allocator::{ MemoryLocation };
use ash::vk::*;

use super::boilerplate::*;
use super::allocator::*;

pub struct GPUbuffers {
  pub channel_input_buff: GpuBuffer,
  pub channel_output_buff: GpuBuffer,
  pub bus_input_buff: GpuBuffer,
  pub bus_output_buff: GpuBuffer
}

pub struct GPU {
  pub base: VKbase,
  pub device: ash::Device,
  pub device_properties: PhysicalDeviceProperties,
  pub channels: u32,
  pub buffer_size: u32,
  pub total_floats: usize,
  pub total_bytes: u64,
  pub allocator: Allocator,
  pub buffers: GPUbuffers
}

impl GPU {
  pub fn new(channels: u32, buffer_size: u32) -> std::result::Result<Self, GPUError> {
    let base = match VKbase::new() {
        Ok(base) => base,
        Err(err) => return Err(err),
    };
    let device = base.device.clone();
    let device_properties = unsafe { base.instance.get_physical_device_properties(base.physical_device) };

    let mut allocator = create_allocator(&base.instance, &base.device, base.physical_device);

    let total_floats: usize = (channels * buffer_size) as usize;
    let total_bytes: u64 = (total_floats * 4) as u64;


    let channel_input_buff = create_buffer(&device, &mut allocator, total_bytes,
    BufferUsageFlags::STORAGE_BUFFER, MemoryLocation::CpuToGpu, "channel_input");

    let channel_output_buff = create_buffer(&device, &mut allocator, total_bytes,
    BufferUsageFlags::STORAGE_BUFFER, MemoryLocation::GpuToCpu, "channel_output");

    let bus_input_buff = create_buffer(&device, &mut allocator, total_bytes,
    BufferUsageFlags::STORAGE_BUFFER, MemoryLocation::CpuToGpu, "bus_input");

    let bus_output_buff = create_buffer(&device, &mut allocator, total_bytes,
    BufferUsageFlags::STORAGE_BUFFER, MemoryLocation::GpuToCpu, "bus_output");

    return Ok(Self {
      base,
      device,
      device_properties,
      channels,
      buffer_size,
      total_floats,
      total_bytes,
      allocator,
      buffers: GPUbuffers { channel_input_buff, channel_output_buff, bus_input_buff, bus_output_buff }
    })
  }

}

