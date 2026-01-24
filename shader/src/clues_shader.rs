use spirv_std::{glam::UVec3, spirv};

#[spirv(compute(threads(64)))]
pub fn check_string_contains(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] clue: &[u8],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] inputs: &mut [u8],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] string_size: &usize,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] output: &mut [u8],
) {
    let index = id.x as usize;
    let begin = index * *string_size;
    let end = index + 1usize * *string_size;

    output[index * *string_size] = contains(inputs, begin, end, clue);
}

fn contains(input: &[u8], begin: usize, end: usize, clue: &[u8]) -> u8 {
    let mut equal = 0;
    for i in 0..clue.len() {
        if input[begin + i] != clue[i] {
            equal = 1;
            break;
        }
        if i >= end - begin {
            break;
        }
    }

    equal
}
