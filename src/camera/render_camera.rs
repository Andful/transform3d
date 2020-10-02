use std::{
    cell::{Ref, RefCell},
    rc::{Rc, Weak},
};

use winit::window::Window;

use cgmath::{prelude::SquareMatrix, Matrix4, Vector3, Vector4};

use crate::{
    Camera, CameraUniform, Component, Light, Mesh, RenderPassWrapper, Scene, State, Transformation,
};

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const OPENGL_TO_WGPU_MATRIX: Matrix4<f32> = Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, 1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
);

pub struct RenderCamera {
    state: State,
    surface: wgpu::Surface,
    size: winit::dpi::PhysicalSize<u32>,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,

    projection_matrix: Matrix4<f32>,

    data: CameraUniform,

    has_parent: bool,
    scene: Weak<RefCell<Scene>>,
    children: Vec<Rc<RefCell<dyn Component>>>,
    transformation: Transformation,
}

impl RenderCamera {
    pub fn new_ref(
        projection_matrix: Matrix4<f32>,
        window: &Window,
        state: &State,
    ) -> Rc<RefCell<RenderCamera>> {
        let size = window.inner_size();

        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let surface = unsafe { state.instance.create_surface(window) };
        let swap_chain = state.device.create_swap_chain(&surface, &sc_desc);

        let data = CameraUniform {
            projection_matrix: OPENGL_TO_WGPU_MATRIX * projection_matrix,
            camera_position: Vector3::<f32>::new(0.0f32, 0.0f32, 0.0f32),
        };

        let result = RenderCamera {
            state: state.clone(),
            surface,
            sc_desc,
            swap_chain,
            size,
            projection_matrix,
            data,
            has_parent: false,
            scene: Weak::new(),
            children: Vec::new(),
            transformation: Transformation::new(),
        };

        Rc::new(RefCell::new(result))
    }

    pub fn resize(&mut self, size: winit::dpi::PhysicalSize<u32>) {
        self.size = size;
        self.sc_desc.width = size.width;
        self.sc_desc.height = size.height;
        self.swap_chain = self
            .state
            .device
            .create_swap_chain(&self.surface, &self.sc_desc);
    }
}

impl Component for RenderCamera {
    fn added_to_scene(&mut self) {
        println!("camera added to scene");
    }
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
    fn update(&mut self) {
        self.data.projection_matrix = OPENGL_TO_WGPU_MATRIX
            * self.projection_matrix
            * self
                .transformation()
                .global_transformation()
                .invert()
                .unwrap();
        let camera_position =
            self.transformation().global_transformation() * Vector4::<f32>::new(0.0, 0.0, 0.0, 1.0);
        self.data.camera_position.x = camera_position.x / camera_position.w;
        self.data.camera_position.y = camera_position.y / camera_position.w;
        self.data.camera_position.z = camera_position.z / camera_position.w;
    }
}

impl Camera for RenderCamera {
    fn render(&mut self) {
        let frame = {
            self.swap_chain
                .get_current_frame()
                .expect("Timeout getting texture")
                .output
        };

        let scene = self.scene.upgrade().unwrap();

        let scene = scene.borrow();

        let mut encoder =
            self.state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        {
            let meshes: Vec<Ref<Mesh>> = scene.meshes.iter().map(|x| x.borrow()).collect();
            let meshes: Vec<&Mesh> = meshes.iter().map(|x| &**x).collect();
            let lights: Vec<Ref<dyn Light>> = scene.lights.iter().map(|x| x.borrow()).collect();
            let lights: Vec<&dyn Light> = lights.iter().map(|x| &**x).collect();
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            for mesh in meshes.iter() {
                mesh.material.draw_material(
                    &*mesh.geometry,
                    &lights,
                    &mesh.render_pipeline,
                    &mut RenderPassWrapper {
                        render_pass: &mut render_pass,
                        mesh_uniform: &mesh.data,
                        camera_uniform: &self.data,
                    },
                );
            }
        }
        self.state.queue.submit(Some(encoder.finish()));
    }
}
