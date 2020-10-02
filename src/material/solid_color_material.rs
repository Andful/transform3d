use wgpu::util::DeviceExt;

use crate::{Bindable, Geometry, Light, Material, RenderPassWrapper, State};

#[derive(Copy, Clone)]
struct SolidColorMaterialData(cgmath::Vector3<f32>);

unsafe impl bytemuck::Zeroable for SolidColorMaterialData {}
unsafe impl bytemuck::Pod for SolidColorMaterialData {}

pub struct SolidColorMaterial {
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
}

impl SolidColorMaterial {
    pub fn new(color: cgmath::Vector3<f32>, state: &State) -> SolidColorMaterial {
        let buffer = state
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Color Buffer"),
                contents: bytemuck::cast_slice(&[SolidColorMaterialData(color)]),
                usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            });

        let bind_group_layout =
            state
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::UniformBuffer {
                            dynamic: false,
                            min_binding_size: std::num::NonZeroU64::new(std::mem::size_of::<
                                SolidColorMaterialData,
                            >(
                            )
                                as u64),
                        },
                        count: None,
                    }],
                    label: Some("solid_color_material_uniform_bind_group_layout".into()),
                });

        let bind_group = state.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 2,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("solid_color_material_uniform_bind_group".into()),
        });

        SolidColorMaterial {
            bind_group_layout,
            bind_group,
        }
    }
}

impl Bindable for SolidColorMaterial {
    fn bind_group(&self) -> Option<&wgpu::BindGroup> {
        Some(&self.bind_group)
    }
}

impl Material for SolidColorMaterial {
    fn fragment_stage(&self) -> wgpu::ShaderModuleSource<'static> {
        wgpu::include_spirv!("solid_color_material.frag.spv")
    }
    fn color_states(&self) -> Vec<wgpu::ColorStateDescriptor> {
        vec![wgpu::ColorStateDescriptor {
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }]
    }
    fn depth_stencil_state(&self) -> Option<wgpu::DepthStencilStateDescriptor> {
        None
    }
    fn sample_count(&self) -> u32 {
        1
    }
    fn sample_mask(&self) -> u32 {
        !0
    }

    fn alpha_to_coverage_enabled(&self) -> bool {
        false
    }

    fn draw_material<'a>(
        &'a self,
        geometry: &'a dyn Geometry,
        _lights: &'a Vec<&dyn Light>,
        pipeline: &'a wgpu::RenderPipeline,
        render_pass: &mut RenderPassWrapper<'a, '_>,
    ) {
        render_pass.set_pipeline(pipeline);
        if let Some(ref bind_group) = geometry.bind_group() {
            render_pass.set_bind_group(0, bind_group, &[]);
        }
        if let Some(ref bind_group) = self.bind_group() {
            render_pass.set_bind_group(0, bind_group, &[]);
        }
        geometry.draw_geometry(render_pass);
    }

    fn material_bind_group_layout(&self) -> Option<&wgpu::BindGroupLayout> {
        Some(&self.bind_group_layout)
    }
}
