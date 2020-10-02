use crate::{Bindable, Geometry, Light, RenderPassWrapper};

pub trait Material: Bindable {
    fn fragment_stage(&self) -> wgpu::ShaderModuleSource;
    fn color_states(&self) -> Vec<wgpu::ColorStateDescriptor>;
    fn depth_stencil_state(&self) -> Option<wgpu::DepthStencilStateDescriptor>;
    fn sample_count(&self) -> u32;
    fn sample_mask(&self) -> u32;
    fn alpha_to_coverage_enabled(&self) -> bool;
    fn draw_material<'a>(
        &'a self,
        geometry: &'a dyn Geometry,
        lights: &'a Vec<&'a dyn Light>,
        pipeline: &'a wgpu::RenderPipeline,
        render_pass: &mut RenderPassWrapper<'a, '_>,
    );
    fn material_bind_group_layout(&self) -> Option<&wgpu::BindGroupLayout>;
}
