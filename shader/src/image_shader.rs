use spirv_std::{
    glam::{vec2, vec4, UVec3, Vec2, Vec4},
    spirv, Image, Sampler,
};

#[spirv(vertex)]
pub fn image_vs(
    position: Vec2,
    tex_coord: &mut Vec2,
    #[spirv(position, invariant)] out_pos: &mut Vec4,
) {
    *out_pos = vec4(position.x, position.y, 0.0, 1.0);
    *tex_coord = position + vec2(0.5, 0.5);
}

#[spirv(fragment)]
pub fn image_fs(
    tex_coord: Vec2,
    #[spirv(descriptor_set = 0, binding = 0)] sampler: &Sampler,
    #[spirv(descriptor_set = 0, binding = 1)] image: &Image!(2D, type=f32, sampled),
    f_color: &mut Vec4,
) {
    *f_color = image.sample(*sampler, tex_coord);
}

// LocalSize/numthreads of (x = 64, y = 1, z = 1)
#[spirv(compute(threads(64)))]
pub fn add_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] left: &mut [u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] right: &mut [u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] output: &mut [u32],
) {
    let index = id.x as usize;
    output[index] = left[index] + right[index];
}
