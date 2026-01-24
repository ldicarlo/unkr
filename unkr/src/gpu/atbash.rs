use crate::gpu::shaders;
use std::{process::Output, sync::Arc};
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage},
    command_buffer::{
        allocator::StandardCommandBufferAllocator, AutoCommandBufferBuilder, CommandBufferUsage,
    },
    descriptor_set::{
        allocator::StandardDescriptorSetAllocator, DescriptorSet, WriteDescriptorSet,
    },
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceFeatures, Queue,
        QueueCreateInfo, QueueFlags,
    },
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
    pipeline::{
        compute::ComputePipelineCreateInfo, layout::PipelineDescriptorSetLayoutCreateInfo,
        ComputePipeline, Pipeline, PipelineBindPoint, PipelineLayout,
        PipelineShaderStageCreateInfo,
    },
    sync::{self, GpuFuture},
    Validated, VulkanError, VulkanLibrary,
};

fn build_compute_pipeline(device: Arc<Device>) -> Arc<ComputePipeline> {
    let cs = shaders::ab::load(device.clone())
        .expect("failed to create shader module")
        .entry_point("atbash_shader::atbash")
        .unwrap();

    let stage = PipelineShaderStageCreateInfo::new(cs);

    let layout = {
        let stages = [stage.clone()];
        PipelineLayout::new(
            device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(stages.iter())
                .into_pipeline_layout_create_info(device.clone())
                .unwrap(),
        )
        .unwrap()
    };

    ComputePipeline::new(
        device,
        None,
        ComputePipelineCreateInfo::stage_layout(stage, layout),
    )
    .expect("failed to create compute pipeline")
}
