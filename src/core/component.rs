use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use cgmath::Matrix4;

use crate::{Scene, Transformation};

pub trait Component {
    //Need to be implemented
    fn has_parent(&self) -> bool;
    fn has_parent_mut(&mut self) -> &mut bool;
    fn scene(&self) -> &Weak<RefCell<Scene>>;
    fn scene_mut(&mut self) -> &mut Weak<RefCell<Scene>>;
    fn children(&self) -> &Vec<Rc<RefCell<dyn Component>>>;
    fn children_mut(&mut self) -> &mut Vec<Rc<RefCell<dyn Component>>>;
    fn transformation(&self) -> &Transformation;
    fn transformation_mut(&mut self) -> &mut Transformation;
    fn add(&mut self, component: Rc<RefCell<dyn Component>>) {
        {
            let mut component = component.borrow_mut();
            if component.has_parent() {
                panic!("Component already has parent");
            }
            *component.has_parent_mut() = true;
            if self.scene().upgrade().is_some() {
                let scene = self.scene().clone();
                component.added_to_scene_recursive(&scene);
            }
        }
        self.children_mut().push(component);
    }

    fn added_to_scene(&mut self) {}

    fn added_to_scene_recursive(&mut self, scene: &Weak<RefCell<Scene>>) {
        *self.scene_mut() = scene.clone();
        self.added_to_scene();
        for child in self.children().iter() {
            let mut child = child.borrow_mut();
            *child.scene_mut() = scene.clone();
            child.added_to_scene_recursive(scene);
        }
    }

    fn update_recursive(&mut self, parent_transformation: &Matrix4<f32>) {
        self.update();
        {
            let transformation = self.transformation_mut();
            transformation.update_local(false);
            transformation.update_global(parent_transformation);
        }
        let transformation = self.transformation();
        for child in self.children().iter() {
            child
                .borrow_mut()
                .update_recursive(transformation.global_transformation());
        }
    }
    fn update(&mut self) {}
}
