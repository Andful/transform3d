use crate::Component;
use cgmath::Matrix4;
use cgmath::Vector3;

#[repr(C, align(16))]
#[derive(Copy, Clone, Debug)]
pub struct CameraUniform {
    pub projection_matrix: Matrix4<f32>,
    pub camera_position: Vector3<f32>,
}

unsafe impl bytemuck::Zeroable for CameraUniform {}
unsafe impl bytemuck::Pod for CameraUniform {}

pub trait Camera: Component {
    fn render(&mut self);
}
