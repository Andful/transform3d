pub trait Bindable {
    fn bind_group(&self) -> Option<&wgpu::BindGroup>;
}
