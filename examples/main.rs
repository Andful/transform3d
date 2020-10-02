use cgmath::{prelude::One, Matrix4};
use futures::executor::block_on;
use transform3d::{
    BaseGeometry, Camera, Component, Group, Mesh, RenderCamera, Scene, SolidColorMaterial, State,
};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use std::path::Path;
use wgpu_subscriber::initialize_default_subscriber;

fn main() {
    initialize_default_subscriber(Some(Path::new("trace.txt")));
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let state = block_on(State::new());
    let scene = Scene::new_ref();

    let camera = RenderCamera::new_ref(Matrix4::<f32>::one(), &window, &state);

    let geometry = BaseGeometry::new(&state);
    let material = SolidColorMaterial::new(cgmath::Vector3::new(0.0, 1.0, 0.0), &state);
    let mesh = Mesh::new_ref(geometry, material, &state);

    let root = Group::new_ref(&scene);

    scene.borrow_mut().set_root(root.clone());

    {
        let mut root = root.borrow_mut();
        root.add(camera.clone());
        root.add(mesh.clone());
        assert!(mesh.borrow().scene().upgrade().is_some());
        assert!(camera.borrow().scene().upgrade().is_some());
    }

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(_) => {
            //state.update();
            camera.borrow_mut().render();
        }
        Event::MainEventsCleared => {
            window.request_redraw();
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {
            if !false {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => match input {
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        _ => {}
                    },
                    WindowEvent::Resized(physical_size) => {
                        camera.borrow_mut().resize(*physical_size);
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        camera.borrow_mut().resize(**new_inner_size);
                    }
                    _ => {}
                }
            }
        }
        _ => {}
    });
}
