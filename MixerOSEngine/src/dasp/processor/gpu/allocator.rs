use ash::{Device, Instance, vk};
use gpu_allocator::MemoryLocation;
use gpu_allocator::vulkan::{
    Allocation, AllocationCreateDesc, AllocationScheme, Allocator, AllocatorCreateDesc,
};
use gpu_allocator::AllocationSizes;

const MB: u64 = 1024 * 1024;

pub struct GpuBuffer {
    pub buffer: vk::Buffer,
    pub allocation: Allocation,
}

pub fn create_allocator(
    instance: &Instance,
    device: &Device,
    physical_device: vk::PhysicalDevice,
) -> Allocator {
    Allocator::new(&AllocatorCreateDesc {
        instance: instance.clone(),
        device: device.clone(),
        physical_device,
        debug_settings: Default::default(),
        buffer_device_address: false,
        allocation_sizes: AllocationSizes::new(8 * MB, 8 * MB)
    })
    .unwrap()
}

pub fn create_buffer(
    device: &Device,
    allocator: &mut Allocator,
    size: u64,
    usage: vk::BufferUsageFlags,
    location: MemoryLocation,
    name: &str,
) -> GpuBuffer {
    let buffer = unsafe {
        device
            .create_buffer(
                &vk::BufferCreateInfo::default()
                    .size(size)
                    .usage(usage)
                    .sharing_mode(vk::SharingMode::EXCLUSIVE),
                None,
            )
            .unwrap()
    };

    let requirements = unsafe { device.get_buffer_memory_requirements(buffer) };

    let allocation = allocator
        .allocate(&AllocationCreateDesc {
            name,
            requirements,
            location,
            linear: true,
            allocation_scheme: AllocationScheme::GpuAllocatorManaged,
        })
        .unwrap();

    unsafe {
        device
            .bind_buffer_memory(buffer, allocation.memory(), allocation.offset())
            .unwrap();
    }

    GpuBuffer { buffer, allocation }
}
