use std::ops::Deref;
use std::rc::Rc;

use crate::CameraUniform;
use crate::LightUniform;
use crate::MeshUniform;

pub struct _State {
    pub instance: wgpu::Instance,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub light_bind_group_layout: wgpu::BindGroupLayout,
}

#[derive(Clone)]
pub struct State(Rc<_State>);

impl Deref for State {
    type Target = _State;
    fn deref(&self) -> &_State {
        &self.0
    }
}

impl State {
    pub async fn new() -> Self {
        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: None,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::PUSH_CONSTANTS | wgpu::Features::DEPTH_CLAMPING,
                    limits: wgpu::Limits {
                        max_push_constant_size: (std::mem::size_of::<CameraUniform>()
                            + std::mem::size_of::<MeshUniform>())
                            as u32,
                        ..wgpu::Limits::default()
                    },
                    shader_validation: true,
                },
                Some(std::path::Path::new("./trace")),
            )
            .await
            .expect("Failed to create device");

        let light_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: std::num::NonZeroU64::new(
                            std::mem::size_of::<LightUniform>() as u64,
                        ),
                    },
                    count: None,
                }],
                label: Some("light_bind_group_layout"),
            });

        Self(Rc::new(_State {
            instance,
            device,
            queue,
            light_bind_group_layout,
        }))
    }
}
