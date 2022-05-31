// inspired by https://github.com/BVE-Reborn/rend3/
use crate::{
  util::{
    log::*,
    //color::RGBA
  },
  vertex::*,
};
use wgpu::{Device, IndexFormat, RenderPass, util::DeviceExt};
use glam::*;
use rgb::{RGBA};

pub enum IndexingStyle {
  FAN,
  SPIRAL
}

pub struct Triangle {
  pub vertices: [Vertex; 3],
}

pub struct MeshBuilder {
  pub vertex_positions: Vec<Vec3>,
  pub vertex_colors: Option<Vec<RGBA<f32>>>,
  pub indices: Vec<u32>, // Make this optional and have default pairing methods maybe?
}

// TODO: add funcitonality for taking only raw verts as tris and constructing vertex and index buffers. (map?)
impl MeshBuilder {
  pub fn new(vertex_positions: Vec<Vec3>, indices: Vec<u32>) -> Self {
    assert!(vertex_positions.len() >= 3);
    Self {
      vertex_positions,
      vertex_colors: None,
      indices,
    }
  }

  pub fn new_quad(vertex_positions: Vec<Vec3>) -> Self {
    assert!(vertex_positions.len() >= 3);
    Self {
      vertex_positions,
      vertex_colors: None,
      indices: vec![
        0, 1, 2,
        0, 2, 3,
      ],
    }
  }

  pub fn new_from_triangles(triangles: Vec<Triangle>) -> Self {
    let mut vertices = Vec::<Vertex>::new();
    let mut indices = Vec::<u32>::new();
    for triangle in triangles.iter() {
      for vertex in triangle.vertices {
        let index = vertices.iter().position(|&v| v == vertex);
        if index.is_none() {
          indices.push(vertices.len() as u32);
          vertices.push(vertex);
        } else {
          indices.push(index.unwrap() as u32);
        }
      }
    }

    Self::new_from_vertices(vertices, indices)
  }

  pub fn new_from_vertices(vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
    assert!(vertices.len() >= 3);
    let vertex_positions: Vec<_> = vertices.iter()
                                           .map(|v| v.position)
                                           .collect();

    // Make this a function
    let vertex_colors = {
      let mut colors: Vec<_> = vertices.iter()
                                       .map(|v| v.color.unwrap_or_else(|| RGBA::new(1.0, 1.0, 1.0, 1.0)))
                                       .collect();

      let first_different_length = [
        (colors.len(), "Color Buffer"),
      ].iter()
       .find_map(|&(len, name)| if len != vertex_positions.len() {
         Some((len, name))
       } else {
         None
       });

      if first_different_length.is_some() {
        error!("Mismatched vertex buffer size: {}", first_different_length.unwrap().1);
        colors.resize(vertex_positions.len(), RGBA::new(1.0, 1.0, 1.0, 1.0));
      }

      Some(colors)
    };

    Self {
      vertex_positions,
      vertex_colors,
      indices,
    }
  }

  pub fn new_poly(vertex_positions: Vec<Vec3>/*, index_style: IndexingStyle*/) -> Self {
    assert!(vertex_positions.len() >= 3);
    let vertex_count = vertex_positions.len() as u32;

    Self {
      vertex_positions,
      vertex_colors: None,
      indices: Self::fanned_indices(vertex_count),
    }
  }

  pub fn new_poly_from_vertices(vertices: Vec<Vertex>) -> Self {
    assert!(vertices.len() >= 3);
    let vertex_count = vertices.len() as u32;

    Self::new_from_vertices(
      vertices,
      Self::fanned_indices(vertex_count)
    )
  }

  fn fanned_indices(vertex_count: u32) -> Vec<u32> {
    let mut indices = Vec::<u32>::new();

    for i in 1..(vertex_count - 1) {
      indices.append(&mut vec![0, i, i + 1]);
    }

    indices
  }

  pub fn with_colors(mut self, vertex_colors: Vec<RGBA<f32>>) -> Self {
    self.vertex_colors = Some(vertex_colors);
    self
  }

  pub fn with_color(mut self, vertex_color: &RGBA<f32>) -> Self {
    self.vertex_colors = Some(vec![*vertex_color; self.vertex_positions.len()]);
    self
  }

  pub fn build(self, device: &Device) -> Mesh {
    let vertex_position_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
      label: Some("Vertex Position Buffer"),
      contents: bytemuck::cast_slice(&self.vertex_positions),
      usage: wgpu::BufferUsages::VERTEX,
    });

    let colors = if self.vertex_colors.is_some() {
      self.vertex_colors.unwrap()
    } else {
      vec![RGBA::new(1.0, 1.0, 1.0, 1.0); self.vertex_positions.len()]
    };
    let vertex_color_buffer = {
      device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Color Buffer"),
        contents: bytemuck::cast_slice(&colors),
        usage: wgpu::BufferUsages::VERTEX,
      })
    };

    let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor{
      label: Some("Index Buffer"),
      contents: bytemuck::cast_slice(&self.indices),
      usage: wgpu::BufferUsages::INDEX,
    });

    Mesh {
      vertex_position_buffer,
      vertex_color_buffer,
      index_buffer,
      index_count: self.indices.len() as u32,
    }
  }
}

pub struct Mesh {
  pub vertex_position_buffer: wgpu::Buffer,
  pub vertex_color_buffer: wgpu::Buffer,
  pub index_buffer: wgpu::Buffer,
  pub index_count: u32,
}

impl Mesh {
  // pub fn new(device: &Device, vertex_positions: Vec<Vec3>, indices: Vec<u32>) -> Self {
  //   MeshBuilder::new(vertex_positions, indices).build(device)
  // }
  //
  // pub fn new_from_vertices(device: &Device, vertices: Vec<Vertex>, indices: Vec<u32>) -> Self {
  //   MeshBuilder::new_from_vertices(vertices, indices).build(device)
  // }
  //
  // pub fn new_poly(device: &Device, vertex_positions: Vec<Vec3>) -> Self {
  //   MeshBuilder::new_poly(vertex_positions).build(device)
  // }
  //
  // pub fn new_poly_from_vertices(device: &Device, vertex_positions: Vec<Vertex>) -> Self {
  //   MeshBuilder::new_poly_from_vertices(vertex_positions).build(device)
  // }

  pub fn bind<'rpass>(&'rpass self, rpass: &mut RenderPass<'rpass>) {
    rpass.set_vertex_buffer(Vertex::VERTEX_POSITION_SLOT, self.vertex_position_buffer.slice(..));
    rpass.set_vertex_buffer(Vertex::VERTEX_COLOR_SLOT, self.vertex_color_buffer.slice(..));
    rpass.set_index_buffer(self.index_buffer.slice(..), IndexFormat::Uint32);
  }

  pub fn draw<'rpass>(&'rpass self, rpass: &mut RenderPass<'rpass>) {
    self.bind(rpass);
    rpass.draw_indexed(0..self.index_count, 0, 0..1);
  }
}