use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use cgmath::{prelude::One, Matrix3, Matrix4};

use crate::{CameraUniform, Component, Geometry, Material, Scene, State, Transformation};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MeshUniform {
    transformation: Matrix4<f32>,
    normal_transformation: Matrix3<f32>,
}

unsafe impl bytemuck::Zeroable for MeshUniform {}
unsafe impl bytemuck::Pod for MeshUniform {}

pub struct Mesh {
    pub render_pipeline: wgpu::RenderPipeline,
    pub geometry: Box<dyn Geometry>,
    pub material: Box<dyn Material>,
    has_parent: bool,
    scene: Weak<RefCell<Scene>>,
    transformation: Transformation,
    children: Vec<Rc<RefCell<dyn Component>>>,
    this: Weak<RefCell<Mesh>>,
    pub data: MeshUniform,
}

impl Mesh {
    pub fn new_ref(
        geometry: impl Geometry + 'static,
        material: impl Material + 'static,
        state: &State,
    ) -> Rc<RefCell<Mesh>> {
        let mut bind_group_layouts = vec![
            //&state.light_bind_group_layout, UNCOMMENT
        ];

        if let Some(bgl) = geometry.geometry_bind_group_layout() {
            bind_group_layouts.push(bgl);
        }
        if let Some(bgl) = material.material_bind_group_layout() {
            bind_group_layouts.push(bgl);
        }

        let render_pipeline_layout =
            state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    bind_group_layouts: &bind_group_layouts,
                    push_constant_ranges: &[wgpu::PushConstantRange {
                        stages: wgpu::ShaderStage::VERTEX | wgpu::ShaderStage::FRAGMENT,
                        range: 0..(std::mem::size_of::<CameraUniform>()
                            + std::mem::size_of::<MeshUniform>())
                            as u32,
                    }],
                    label: None,
                });

        let vs_module = state.device.create_shader_module(geometry.vertex_stage());
        let fs_module = state.device.create_shader_module(material.fragment_stage());

        let render_pipeline =
            state
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    layout: Some(&render_pipeline_layout),
                    vertex_stage: wgpu::ProgrammableStageDescriptor {
                        module: &vs_module,
                        entry_point: "main",
                    },
                    fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                        module: &fs_module,
                        entry_point: "main",
                    }),
                    rasterization_state: Some(geometry.rasterization_state()),
                    color_states: &material.color_states(),
                    primitive_topology: geometry.primitive_topology(),
                    depth_stencil_state: material.depth_stencil_state(),
                    vertex_state: wgpu::VertexStateDescriptor {
                        index_format: geometry.index_format(),
                        vertex_buffers: &geometry.vertex_buffer(),
                    },
                    sample_count: material.sample_count(),
                    sample_mask: material.sample_mask(),
                    alpha_to_coverage_enabled: material.alpha_to_coverage_enabled(),
                    label: None,
                });
        let mesh = Rc::new(RefCell::new(Mesh {
            render_pipeline,
            geometry: Box::new(geometry),
            material: Box::new(material),
            has_parent: false,
            scene: Weak::new(),
            transformation: Transformation::new(),
            children: Vec::new(),
            this: Weak::new(),
            data: MeshUniform {
                transformation: Matrix4::one(),
                normal_transformation: Matrix3::one(),
            },
        }));

        mesh.borrow_mut().this = Rc::downgrade(&mesh);
        mesh
    }
}

impl Component for Mesh {
    fn has_parent(&self) -> bool {
        self.has_parent
    }
    fn has_parent_mut(&mut self) -> &mut bool {
        &mut self.has_parent
    }
    fn scene(&self) -> &Weak<RefCell<Scene>> {
        &self.scene
    }
    fn scene_mut(&mut self) -> &mut Weak<RefCell<Scene>> {
        &mut self.scene
    }
    fn children(&self) -> &Vec<Rc<RefCell<dyn Component>>> {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<Rc<RefCell<dyn Component>>> {
        &mut self.children
    }
    fn transformation(&self) -> &Transformation {
        &self.transformation
    }
    fn transformation_mut(&mut self) -> &mut Transformation {
        &mut self.transformation
    }
    fn added_to_scene(&mut self) {
        self.scene
            .upgrade()
            .unwrap()
            .borrow_mut()
            .meshes
            .push(self.this.upgrade().unwrap());
    }
}
