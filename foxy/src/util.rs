#[macro_use]
pub mod log;
pub mod error;

#[macro_use]
pub mod shaders {
  #[macro_export]
  macro_rules! read_shader {
    ( $shader:literal; $device:expr ) => {{
      use wgpu::include_wgsl;
      $device.create_shader_module(&include_wgsl!(
        concat!("../res/shaders/", $shader, ".wgsl")
      ))
    }}
  }
}