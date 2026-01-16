use spirv_std::{glam::UVec3, spirv};

#[spirv(compute(threads(64)))]
pub fn atbash(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] input: &mut [&[u8]],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] output: &mut [&mut [u8]],
) {
    let index = id.x as usize;
    let out: &mut [u8] = output[index];
    for (i, idx) in input[index].into_iter().enumerate() {
        out[i] = *idx + 1;
    }
}
