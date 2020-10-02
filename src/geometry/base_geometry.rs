use std::mem;

use wgpu::util::DeviceExt;

use cgmath::Vector3;

use crate::{Bindable, Geometry, State};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: Vector3<f32>,
}

unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: Vector3::<f32>::new(-0.0868241, 0.49240386, 0.0),
    },
    Vertex {
        position: Vector3::<f32>::new(-0.49513406, 0.06958647, 0.0),
    },
    Vertex {
        position: Vector3::<f32>::new(-0.21918549, -0.44939706, 0.0),
    },
    Vertex {
        position: Vector3::<f32>::new(0.35966998, -0.3473291, 0.0),
    },
    Vertex {
        position: Vector3::<f32>::new(0.44147372, 0.2347359, 0.0),
    },
];

const INDICES: &[u16] = &[0, 1, 4, 1, 2, 4, 2, 3, 4];

pub struct BaseGeometry {
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl BaseGeometry {
    pub fn new(state: &State) -> BaseGeometry {
        BaseGeometry {
            vertex_buffer: state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Vertex Buffer"),
                    contents: bytemuck::cast_slice(VERTICES),
                    usage: wgpu::BufferUsage::VERTEX,
                }),
            index_buffer: state
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(INDICES),
                    usage: wgpu::BufferUsage::INDEX,
                }),
            num_indices: INDICES.len() as u32,
        }
    }
}

impl Bindable for BaseGeometry {
    fn bind_group(&self) -> Option<&wgpu::BindGroup> {
        None
    }
}

impl Geometry for BaseGeometry {
    fn vertex_stage(&self) -> wgpu::ShaderModuleSource<'static> {
        wgpu::include_spirv!("base_geometry.vert.spv")
    }
    fn rasterization_state(&self) -> wgpu::RasterizationStateDescriptor {
        wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            clamp_depth: true,
            cull_mode: wgpu::CullMode::Back,
            polygon_mode: wgpu::PolygonMode::Fill,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }
    }
    fn primitive_topology(&self) -> wgpu::PrimitiveTopology {
        wgpu::PrimitiveTopology::TriangleList
    }
    fn index_format(&self) -> wgpu::IndexFormat {
        wgpu::IndexFormat::Uint16
    }
    fn vertex_buffer(&self) -> Vec<wgpu::VertexBufferDescriptor> {
        vec![wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[wgpu::VertexAttributeDescriptor {
                offset: 0,
                shader_location: 0,
                format: wgpu::VertexFormat::Float3,
            }],
        }]
    }

    fn draw_geometry<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..));
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }

    fn geometry_bind_group_layout(&self) -> Option<&wgpu::BindGroupLayout> {
        None
    }
}
