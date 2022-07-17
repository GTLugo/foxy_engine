use std::mem::size_of;
use glam::*;
use rgb::{RGBA};
use wgpu::{vertex_attr_array, VertexAttribute};

#[derive(Copy, Clone, Debug, PartialEq)]
#[repr(C)]
pub struct Vertex {
  pub position: Vec3,
  pub color: Option<RGBA<f32>>,
}

impl Vertex {
  pub const VERTEX_POSITION_SLOT: u32 = 0;
  pub const VERTEX_COLOR_SLOT: u32 = 1;

  const POSITION_ATTRIBS: [VertexAttribute; 1] = vertex_attr_array![Self::VERTEX_POSITION_SLOT => Float32x3];
  const COLOR_ATTRIBS: [VertexAttribute; 1] = vertex_attr_array![Self::VERTEX_COLOR_SLOT => Float32x4];
}

pub static VERTEX_BUFFER_LAYOUTS: [wgpu::VertexBufferLayout<'static>; 2] = [
  wgpu::VertexBufferLayout {
    array_stride: size_of::<Vec3>() as wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &Vertex::POSITION_ATTRIBS,
  },
  wgpu::VertexBufferLayout {
    array_stride: size_of::<RGBA<f32>>() as wgpu::BufferAddress,
    step_mode: wgpu::VertexStepMode::Vertex,
    attributes: &Vertex::COLOR_ATTRIBS,
  }
];
