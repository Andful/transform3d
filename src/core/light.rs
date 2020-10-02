use cgmath::Matrix4;
use cgmath::Vector3;

use crate::Component;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct LightUniform {
    light_color: Vector3<f32>,
    light_position: Vector3<f32>,
    is_directional_light: u32,
    light_projection_matrix: Matrix4<f32>,
}

unsafe impl bytemuck::Zeroable for LightUniform {}
unsafe impl bytemuck::Pod for LightUniform {}

pub trait Light: Component {
    fn bind_light(&self, render_pass: &wgpu::RenderPass);
}
