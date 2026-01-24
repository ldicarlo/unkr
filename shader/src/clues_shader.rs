use spirv_std::{glam::UVec3, spirv};

#[spirv(compute(threads(64)))]
pub fn check_string_contains(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] clue: &[u8],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] inputs: &mut [u8],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] string_size: usize,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] output: &mut [bool],
) {
    let index = id.x as usize;

    let slice = &inputs[(index * string_size)..(index + 1) * string_size];

    output[index * string_size] = contains(slice, clue);
}

fn contains(input: &[u8], clue: &[u8]) -> bool {
    if input == clue {
        true
    } else {
        false
    }
}
