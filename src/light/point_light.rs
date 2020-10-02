use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{Component, Light, Scene, Transformation};

pub struct PointLight {
    has_parent: bool,
    scene: Weak<RefCell<Scene>>,
    transformation: Transformation,
    children: Vec<Rc<RefCell<dyn Component>>>,
}

impl Component for PointLight {
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
}

impl Light for PointLight {
    fn bind_light(&self, _render_pass: &wgpu::RenderPass) {
        unimplemented!();
    }
}
