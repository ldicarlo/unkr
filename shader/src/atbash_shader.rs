use spirv_std::{glam::UVec3, spirv};

#[spirv(compute(threads(64)))]
pub fn atbash(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] input: &mut [u8],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] output: &mut [u8],
) {
    let index = id.x as usize;

    for i in 0..255 {
        output[index * 255 + i] = 65 + 25 - ((input[index * 255 + i] - 65) % 26);
    }
}
