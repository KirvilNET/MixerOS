use ash::{vk, Device};

pub struct BufferBinding {
  pub binding: u32,
  pub buffer: vk::Buffer,
  pub size: u64,
}

pub struct PushConstants {
    pub data: Vec<u8>,
}

impl PushConstants {
  pub fn new<T: Copy>(value: &T) -> Self {
    let data = unsafe {
      std::slice::from_raw_parts(
        value as *const T as *const u8,
        std::mem::size_of::<T>(),
      ).to_vec()
    };

    Self { 
      data 
    }
  }

  pub fn empty() -> Self {
      Self { 
        data: vec![] 
      }
  }
}

#[derive(Debug, Clone)]
pub struct ComputePipeline {
    pipeline: vk::Pipeline,
    pipeline_layout: vk::PipelineLayout,
    descriptor_set_layout: vk::DescriptorSetLayout,
    descriptor_pool: vk::DescriptorPool,
    command_pool: vk::CommandPool,
    num_bindings: u32,
}

impl ComputePipeline {
    /// Create a pipeline from a compiled .spv path.
    /// `num_bindings` = number of storage buffer bindings your shader uses.
    /// `push_constant_size` = std::mem::size_of::<YourCoeffsStruct>(), or 0 if none.
    pub fn new(
        device: &Device,
        queue_family_index: u32,
        spv_path: &str,
        num_bindings: u32,
        push_constant_size: u32,
    ) -> Self {
        let descriptor_set_layout = Self::create_dsl(device, num_bindings);
        let (pipeline, pipeline_layout) =
            Self::create_pipeline(device, descriptor_set_layout, spv_path, push_constant_size);
        let descriptor_pool = Self::create_pool(device, num_bindings);
        let command_pool = unsafe {
            device.create_command_pool(
                &vk::CommandPoolCreateInfo::default()
                    .queue_family_index(queue_family_index)
                    .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER),
                None,
            ).unwrap()
        };

        Self {
            pipeline,
            pipeline_layout,
            descriptor_set_layout,
            descriptor_pool,
            command_pool,
            num_bindings,
        }
    }

    fn create_dsl(device: &Device, num_bindings: u32) -> vk::DescriptorSetLayout {
        let bindings: Vec<vk::DescriptorSetLayoutBinding> = (0..num_bindings)
            .map(|i| {
                vk::DescriptorSetLayoutBinding::default()
                    .binding(i)
                    .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                    .descriptor_count(1)
                    .stage_flags(vk::ShaderStageFlags::COMPUTE)
            })
            .collect();

        unsafe {
            device.create_descriptor_set_layout(
                &vk::DescriptorSetLayoutCreateInfo::default().bindings(&bindings),
                None,
            ).unwrap()
        }
    }

    fn create_pipeline(
        device: &Device,
        dsl: vk::DescriptorSetLayout,
        spv_path: &str,
        push_constant_size: u32,
    ) -> (vk::Pipeline, vk::PipelineLayout) {
        let spv = std::fs::read(spv_path)
            .unwrap_or_else(|_| panic!("Could not read shader: {}", spv_path));
        let spv_u32 = ash::util::read_spv(&mut std::io::Cursor::new(spv)).unwrap();

        let shader_module = unsafe {
            device.create_shader_module(
                &vk::ShaderModuleCreateInfo::default().code(&spv_u32),
                None,
            ).unwrap()
        };

        // Only attach push constant range if the shader actually uses one
        let push_ranges = if push_constant_size > 0 {
            vec![vk::PushConstantRange::default()
                .stage_flags(vk::ShaderStageFlags::COMPUTE)
                .offset(0)
                .size(push_constant_size)]
        } else {
            vec![]
        };

        let pipeline_layout = unsafe {
            device.create_pipeline_layout(
                &vk::PipelineLayoutCreateInfo::default()
                    .set_layouts(&[dsl])
                    .push_constant_ranges(&push_ranges),
                None,
            ).unwrap()
        };

        let entry = std::ffi::CString::new("main").unwrap();
        let stage = vk::PipelineShaderStageCreateInfo::default()
            .stage(vk::ShaderStageFlags::COMPUTE)
            .module(shader_module)
            .name(&entry);

        let pipeline = unsafe {
            device.create_compute_pipelines(
                vk::PipelineCache::null(),
                &[vk::ComputePipelineCreateInfo::default()
                    .stage(stage)
                    .layout(pipeline_layout)],
                None,
            ).unwrap()[0]
        };

        unsafe { device.destroy_shader_module(shader_module, None) };
        (pipeline, pipeline_layout)
    }

    fn create_pool(device: &Device, num_bindings: u32) -> vk::DescriptorPool {
        let pool_size = vk::DescriptorPoolSize::default()
            .ty(vk::DescriptorType::STORAGE_BUFFER)
            .descriptor_count(num_bindings);

        unsafe {
            device.create_descriptor_pool(
                &vk::DescriptorPoolCreateInfo::default()
                    .max_sets(1)
                    .pool_sizes(&[pool_size]),
                None,
            ).unwrap()
        }
    }

    /// Dispatch the shader.
    /// - `bindings`: one entry per buffer binding, in binding order
    /// - `push_constants`: use PushConstants::new(&my_struct) or PushConstants::empty()
    /// - `groups`: (x, y, z) workgroup counts
    pub fn dispatch(
        &self,
        device: &Device,
        compute_queue: vk::Queue,
        bindings: &[BufferBinding],
        push_constants: &PushConstants,
        groups: (u32, u32, u32),
    ) {
        assert_eq!(
            bindings.len(), self.num_bindings as usize,
            "Wrong number of buffer bindings: expected {}, got {}",
            self.num_bindings, bindings.len()
        );

        // Write descriptor set
        let ds = unsafe {
            device.allocate_descriptor_sets(
                &vk::DescriptorSetAllocateInfo::default()
                    .descriptor_pool(self.descriptor_pool)
                    .set_layouts(&[self.descriptor_set_layout]),
            ).unwrap()[0]
        };

        let buffer_infos: Vec<vk::DescriptorBufferInfo> = bindings
            .iter()
            .map(|b| {
                vk::DescriptorBufferInfo::default()
                    .buffer(b.buffer)
                    .offset(0)
                    .range(b.size)
            })
            .collect();

        let writes: Vec<vk::WriteDescriptorSet> = bindings
            .iter()
            .enumerate()
            .map(|(i, b)| {
                vk::WriteDescriptorSet::default()
                    .dst_set(ds)
                    .dst_binding(b.binding)
                    .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
                    .buffer_info(std::slice::from_ref(&buffer_infos[i]))
            })
            .collect();

        unsafe { device.update_descriptor_sets(&writes, &[]) };

        // Record and submit
        let cmd = unsafe {
            device.allocate_command_buffers(
                &vk::CommandBufferAllocateInfo::default()
                    .command_pool(self.command_pool)
                    .level(vk::CommandBufferLevel::PRIMARY)
                    .command_buffer_count(1),
            ).unwrap()[0]
        };

        unsafe {
            device.begin_command_buffer(
                cmd,
                &vk::CommandBufferBeginInfo::default()
                    .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT),
            ).unwrap();

            device.cmd_bind_pipeline(cmd, vk::PipelineBindPoint::COMPUTE, self.pipeline);

            device.cmd_bind_descriptor_sets(
                cmd,
                vk::PipelineBindPoint::COMPUTE,
                self.pipeline_layout,
                0,
                &[ds],
                &[],
            );

            if !push_constants.data.is_empty() {
                device.cmd_push_constants(
                    cmd,
                    self.pipeline_layout,
                    vk::ShaderStageFlags::COMPUTE,
                    0,
                    &push_constants.data,
                );
            }

            device.cmd_dispatch(cmd, groups.0, groups.1, groups.2);
            device.end_command_buffer(cmd).unwrap();

            device.queue_submit(
                compute_queue,
                &[vk::SubmitInfo::default().command_buffers(&[cmd])],
                vk::Fence::null(),
            ).unwrap();
            device.queue_wait_idle(compute_queue).unwrap();
        }
    }

    pub fn destroy(&self, device: &Device) {
        unsafe {
            device.destroy_pipeline(self.pipeline, None);
            device.destroy_pipeline_layout(self.pipeline_layout, None);
            device.destroy_descriptor_set_layout(self.descriptor_set_layout, None);
            device.destroy_descriptor_pool(self.descriptor_pool, None);
            device.destroy_command_pool(self.command_pool, None);
        }
    }
}