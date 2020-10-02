use crate::Bindable;

pub struct GeometryUniformBase {
    pub has_normals: bool,
    pub has_color_vertices: bool,
    pub has_tex_coordinates: bool,
}

pub trait Geometry: Bindable {
    fn vertex_stage(&self) -> wgpu::ShaderModuleSource;
    fn rasterization_state(&self) -> wgpu::RasterizationStateDescriptor;
    fn primitive_topology(&self) -> wgpu::PrimitiveTopology;
    fn index_format(&self) -> wgpu::IndexFormat;
    fn vertex_buffer(&self) -> Vec<wgpu::VertexBufferDescriptor>;
    fn draw_geometry<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>);
    fn geometry_bind_group_layout(&self) -> Option<&wgpu::BindGroupLayout>;
}
