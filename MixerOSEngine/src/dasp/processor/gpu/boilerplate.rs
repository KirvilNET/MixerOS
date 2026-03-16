use ash::{ Device, Entry, vk };

pub enum GPUError {
    NoGPU,

}

#[derive(Clone)]
pub struct VKbase {
    pub entry: Entry,
    pub instance: ash::Instance,
    pub physical_device: vk::PhysicalDevice,
    pub device: Device,
    pub compute_queue: vk::Queue,
    pub queue_family_index: u32,
}

impl VKbase {
    pub fn new() -> Self {
        let entry = Entry::linked();

        let app_info = vk::ApplicationInfo {
            api_version: vk::make_api_version(0, 1, 0, 0),
            ..Default::default()
        };

        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            ..Default::default()
        };

        let layer_names: Vec<*const i8> = if cfg!(debug_assertions) {
            vec![c"VK_LAYER_KHRONOS_validation".as_ptr()]
        } else {
            vec![]
        };

        let instance = unsafe {
            entry
                .create_instance(
                    &create_info
                        .application_info(&app_info)
                        .enabled_layer_names(&layer_names),
                    None,
                )
                .expect("Failed to create instance")
        };

        let (physical_device, queue_family_index) = unsafe {
            instance
                .enumerate_physical_devices()
                .unwrap()
                .into_iter()
                .find_map(|pd| {
                    instance
                        .get_physical_device_queue_family_properties(pd)
                        .iter()
                        .enumerate()
                        .find_map(|(i, props)| {
                            if props.queue_flags.contains(vk::QueueFlags::COMPUTE) {
                                Some((pd, i as u32))
                            } else {
                                None
                            }
                        })
                })
                .expect("No device with compute support found")
        };

        // --- Logical device ---
        let queue_priority = [1.0f32];
        let queue_info = vk::DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index)
            .queue_priorities(&queue_priority);

        let device = unsafe {
            instance
                .create_device(
                    physical_device,
                    &vk::DeviceCreateInfo::default().queue_create_infos(&[queue_info]),
                    None,
                )
                .expect("Failed to create logical device")
        };

        let compute_queue = unsafe { device.get_device_queue(queue_family_index, 0) };

        Self {
            entry,
            instance,
            physical_device,
            device,
            compute_queue,
            queue_family_index,
        }
    }

    pub fn check() -> Result<(), GPUError> {
        let entry = Entry::linked();

        let app_info = vk::ApplicationInfo {
            api_version: vk::make_api_version(0, 1, 0, 0),
            ..Default::default()
        };

        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            ..Default::default()
        };

        let layer_names: Vec<*const i8> = if cfg!(debug_assertions) {
            vec![c"VK_LAYER_KHRONOS_validation".as_ptr()]
        } else {
            vec![]
        };

        unsafe {
            match entry.create_instance(
                &create_info
                .application_info(&app_info)
                .enabled_layer_names(&layer_names),
                None,
            ) {
                Ok(inst) => {
                        let gpu = inst
                            .enumerate_physical_devices()
                            .unwrap()
                            .into_iter()
                            .find_map(|pd| {
                                inst
                                    .get_physical_device_queue_family_properties(pd)
                                    .iter()
                                    .enumerate()
                                    .find_map(|(i, props)| {
                                        if props.queue_flags.contains(vk::QueueFlags::COMPUTE) {
                                            Some((pd, i as u32))
                                        } else {
                                            None
                                        }
                                    })
                            })
                            .is_none();

                        if gpu {
                            return Err(GPUError::NoGPU)
                        } else {
                            return Ok(())
                        }
                    },
                Err(_) => return Err(GPUError::NoGPU)
            }
        };
    }
}

impl Drop for VKbase {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_device(None);
            self.instance.destroy_instance(None);
        }
    }
}
