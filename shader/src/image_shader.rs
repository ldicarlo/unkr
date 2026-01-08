use glam::UVec3;
use spirv_std::glam::{vec2, vec4, Vec2, Vec4};
use spirv_std::{glam, spirv, Image, Sampler};

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

pub fn collatz(mut n: u32) -> Option<u32> {
    let mut i = 0;
    if n == 0 {
        return None;
    }
    while n != 1 {
        n = if n.is_multiple_of(2) {
            n / 2
        } else {
            // Overflow? (i.e. 3*n + 1 > 0xffff_ffff)
            if n >= 0x5555_5555 {
                return None;
            }
            // TODO: Use this instead when/if checked add/mul can work:
            // n.checked_mul(3)?.checked_add(1)?
            3 * n + 1
        };
        i += 1;
    }
    Some(i)
}

// LocalSize/numthreads of (x = 64, y = 1, z = 1)
#[spirv(compute(threads(64)))]
pub fn main_cs(
    #[spirv(global_invocation_id)] id: UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] prime_indices: &mut [u32],
) {
    let index = id.x as usize;
    prime_indices[index] = collatz(prime_indices[index]).unwrap_or(u32::MAX);
}
