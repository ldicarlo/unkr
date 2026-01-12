#[cfg(not(feature = "use-glsl-shader"))]
pub mod vs {
    vulkano_shaders::shader! {
        root_path_env: "SHADER_OUT_DIR",
        bytes: "image_shader-image_vs.spv",
    }
}

#[cfg(not(feature = "use-glsl-shader"))]
pub mod fs {
    vulkano_shaders::shader! {
        root_path_env: "SHADER_OUT_DIR",
        bytes: "image_shader-image_fs.spv",
    }
}

#[cfg(not(feature = "use-glsl-shader"))]
pub mod cs {
    vulkano_shaders::shader! {
        root_path_env: "SHADER_OUT_DIR",
        bytes: "image_shader-add_cs.spv",
    }
}
