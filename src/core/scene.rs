use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{Component, Hitbox, Light, Mesh};

pub struct Scene {
    this: Weak<RefCell<Scene>>,
    root: Option<Rc<RefCell<dyn Component>>>,

    pub meshes: Vec<Rc<RefCell<Mesh>>>,
    pub lights: Vec<Rc<RefCell<dyn Light>>>,
    pub hitboxes: Vec<Rc<RefCell<Hitbox>>>,
}

impl Scene {
    pub fn new_ref() -> Rc<RefCell<Scene>> {
        let scene = Scene {
            this: Weak::new(),
            root: None,

            meshes: Vec::new(),
            lights: Vec::new(),
            hitboxes: Vec::new(),
        };

        let scene = Rc::new(RefCell::new(scene));
        scene.borrow_mut().this = Rc::downgrade(&scene);
        scene
    }

    pub fn set_root(&mut self, root: Rc<RefCell<dyn Component>>) {
        {
            let mut root = root.borrow_mut();
            if root.has_parent() {
                panic!("Root cannot have parent");
            }
            *root.has_parent_mut() = true;
            root.added_to_scene_recursive(&self.this);
        }
        self.root = Some(root);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        use futures::executor::block_on;
        async fn run() {
            let _instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
            let _scene = Scene::new_ref();
        }
        block_on(run());
    }
}
