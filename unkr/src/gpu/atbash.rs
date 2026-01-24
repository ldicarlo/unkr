use crate::gpu::shaders;
use std::sync::Arc;
use vulkano::{
    device::Device,
    pipeline::{
        compute::ComputePipelineCreateInfo, layout::PipelineDescriptorSetLayoutCreateInfo,
        ComputePipeline, PipelineLayout, PipelineShaderStageCreateInfo,
    },
};

pub fn build_compute_pipeline(device: Arc<Device>) -> Arc<ComputePipeline> {
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
