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

  // pub fn new_from_triangles(vertex_positions: Vec<Triangle>) -> Self {
  //   assert!(vertex_positions.len() >= 3);
  //   Self {
  //     vertex_positions,
  //     vertex_colors: None,
  //     indices: vec![
  //       0, 1, 2,
  //       0, 2, 3,
  //     ],
  //   }
  // }

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
      indices: Self::get_indices(vertex_count, IndexingStyle::FAN),
    }
  }

  pub fn new_poly_from_vertices(vertices: Vec<Vertex>/*, index_style: IndexingStyle*/) -> Self {
    assert!(vertices.len() >= 3);
    let vertex_count = vertices.len() as u32;

    Self::new_from_vertices(
      vertices,
      Self::get_indices(vertex_count, IndexingStyle::FAN)
    )
  }

  fn get_indices(vertex_count: u32, index_style: IndexingStyle) -> Vec<u32> {
    match index_style {
      IndexingStyle::FAN => {
        Self::fanned_indices(vertex_count)
      }
      IndexingStyle::SPIRAL => {
        Self::spiral_indices(vertex_count)
      }
    }
  }

  fn fanned_indices(vertex_count: u32) -> Vec<u32> {
    let mut indices = Vec::<u32>::new();

    for i in 1..(vertex_count - 1) {
      indices.append(&mut vec![0, i, i + 1]);
    }

    indices
  }

  /*fn spiral_indices_old(vertex_count: u32) -> Vec<u32> {
    let mut indices = Vec::<u32>::new();
    debug!("START");

    let mut skip_factor = 1;
    let mut j = 0;
    let mut should_iterate = true;
    let mut seen = true;
    for _tri_count in 0..(vertex_count - 2) as u32 {
      fn get_index(
        element: u32,
        j: &mut u32,
        skip_factor: &mut u32,
        size: u32,
        seen: &mut bool
      ) -> u32 {
        let x = ((*j * 2 + element) * *skip_factor) % size;

        if x == 0 {
          if !*seen {
            *seen = true;
            *skip_factor *= 2;
            *j = 0;
          } else {
            *seen = false;
          }
        }
        x
        //debug!("x: {}, y: {}", x, (x % size) * *skip_factor);
      }

      let a = get_index(0, &mut j, &mut skip_factor, vertex_count, &mut seen);
      let b = get_index(1, &mut j, &mut skip_factor, vertex_count, &mut seen);
      let c = get_index(2, &mut j, &mut skip_factor, vertex_count, &mut seen);

      j += 1;

      indices.append(&mut vec![a, b, c]);
      debug!("{:?}", vec![a, b, c]);
    }

    indices
  }*/

  fn spiral_indices(vertex_count: u32) -> Vec<u32> {
    let mut indices = Vec::<u32>::new();

    let first_index = 0;
    indices.push(first_index);

    let mut double = false; // repeat value
    let mut increment = 1; // amount to increase value for next value
    let mut _loopback_increment = 0; // additional offset for next value upon finished loop in spiral

    let mut x = 1;
    let mut i = 1;
    let end = ((vertex_count - 2) * 3) - 1; // end of loop
    while i <= end {
      let index = x % vertex_count;

      indices.push(index);
      if double && i != end {
        indices.push(index);
        double = false;
        i += 1;
        // debug!("{}", index);
      } else {
        double = true;
      }

      // debug!("x {}, index {}, increment {}, next {}", x, index, increment, (index + increment) >= vertex_count);
      x += increment;
      let next_index = x % vertex_count;
      if index + increment >= vertex_count {
        debug!("x {}, index {}, next_index {}, increment {}, vertex_count {}", x, index, next_index, increment, vertex_count);
        x += (vertex_count - next_index) % 2;
        _loopback_increment += 1;
        increment *= 2;
      }
      i += 1;
    }

    debug!("{:?}", indices);

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