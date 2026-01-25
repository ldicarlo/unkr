use crate::gpu::{self};
use std::sync::Arc;
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
    pipeline::{Pipeline, PipelineBindPoint},
    sync::{self, GpuFuture},
    Validated, VulkanError, VulkanLibrary,
};

const LOCAL_SIZE_X: u32 = 64;

///

pub fn run_gpu(
    str: String,
    clues: Vec<String>,
    decryptors: Vec<String>,
    threads_number: Vec<u8>,
    total_threads: u8,
    cache_name: String,
    pretty: bool,
    intermediate_steps: bool,
) {
    let (device, queue) = init_device();
    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
    let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
        device.clone(),
        Default::default(),
    ));
    let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
        device.clone(),
        Default::default(),
    ));

    let lhs_data: Vec<u8> = str
        .to_uppercase()
        .as_bytes()
        .into_iter()
        .map(|s| *s)
        .collect();

    let lhs_buffer = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::STORAGE_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        lhs_data.clone(),
    )
    .unwrap();

    let output_buffer = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::STORAGE_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_HOST
                | MemoryTypeFilter::HOST_RANDOM_ACCESS,
            ..Default::default()
        },
        (0..lhs_data.len()).map(|_| 0u8),
    )
    .unwrap();

    let pipeline = gpu::atbash::build_compute_pipeline(device.clone());
    let descriptor_set = DescriptorSet::new(
        descriptor_set_allocator.clone(),
        pipeline.layout().set_layouts().first().unwrap().clone(),
        [
            WriteDescriptorSet::buffer(0, lhs_buffer.clone()),
            WriteDescriptorSet::buffer(1, output_buffer.clone()),
        ],
        [],
    )
    .unwrap();

    let mut builder = AutoCommandBufferBuilder::primary(
        command_buffer_allocator.clone(),
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    let workgroups = ((lhs_data.len() as u32) + LOCAL_SIZE_X - 1) / LOCAL_SIZE_X;

    builder
        .bind_pipeline_compute(pipeline.clone())
        .unwrap()
        .bind_descriptor_sets(
            PipelineBindPoint::Compute,
            pipeline.layout().clone(),
            0,
            descriptor_set,
        )
        .unwrap();
    unsafe { builder.dispatch([workgroups, 1, 1]) }.unwrap();

    let command_buffer = builder.build().unwrap();

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush();

    match future.map_err(Validated::unwrap) {
        Ok(future) => {
            future.wait(None).unwrap();
        }
        Err(VulkanError::OutOfDate) => {
            panic!("Device became out of date while running compute workload");
        }
        Err(e) => panic!("failed to run compute shader: {e}"),
    }

    let results = output_buffer.read().unwrap();
    let chunks = results.as_chunks::<255>();
    for chunk in chunks.0.into_iter() {
        let str = str::from_utf8(chunk).unwrap();
        println!("{str}");
    }
    let str = str::from_utf8(chunks.1).unwrap();
    println!("{str}");
}

fn init_device() -> (Arc<Device>, Arc<Queue>) {
    let library = VulkanLibrary::new().unwrap();
    let instance = Instance::new(library, InstanceCreateInfo::default()).unwrap();

    let (physical_device, queue_family_index) = instance
        .enumerate_physical_devices()
        .unwrap()
        .filter_map(|physical_device| {
            physical_device
                .queue_family_properties()
                .iter()
                .enumerate()
                .position(|(_, q)| q.queue_flags.intersects(QueueFlags::COMPUTE))
                .map(|i| (physical_device, i as u32))
        })
        .min_by_key(|(p, _)| match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
            _ => 5,
        })
        .expect("no device available to run compute workloads");

    println!(
        "Using device: {} (type: {:?})",
        physical_device.properties().device_name,
        physical_device.properties().device_type,
    );

    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            enabled_features: DeviceFeatures {
                vulkan_memory_model: true,
                shader_int8: true,
                ..DeviceFeatures::empty()
            },
            ..Default::default()
        },
    )
    .unwrap();

    (device.clone(), queues.next().unwrap())
}
