use std::ops::{Deref, DerefMut};

use crate::CameraUniform;
use crate::MeshUniform;

pub struct RenderPassWrapper<'a, 'b> {
    pub render_pass: &'b mut wgpu::RenderPass<'a>,
    pub mesh_uniform: &'a MeshUniform,
    pub camera_uniform: &'a CameraUniform,
}

impl<'a> RenderPassWrapper<'a, '_> {
    pub fn set_pipeline(&mut self, pipeline: &'a wgpu::RenderPipeline) {
        self.render_pass.set_pipeline(pipeline);

        self.render_pass.set_push_constants(
            wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
            0,
            bytemuck::cast_slice(&[*self.camera_uniform]),
        );

        self.render_pass.set_push_constants(
            wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
            std::mem::size_of::<CameraUniform>() as u32,
            bytemuck::cast_slice(&[*self.mesh_uniform]),
        );
    }
}

impl<'a> Deref for RenderPassWrapper<'a, '_> {
    type Target = wgpu::RenderPass<'a>;

    fn deref(&self) -> &Self::Target {
        &self.render_pass
    }
}

impl<'a> DerefMut for RenderPassWrapper<'a, '_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.render_pass
    }
}
