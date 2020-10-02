use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{Component, Scene, Transformation};

pub struct Group {
    has_parent: bool,
    scene: Weak<RefCell<Scene>>,
    transformation: Transformation,
    children: Vec<Rc<RefCell<dyn Component>>>,
}

impl Group {
    pub fn new_ref(scene: &Rc<RefCell<Scene>>) -> Rc<RefCell<Group>> {
        Rc::new(RefCell::new(Group {
            has_parent: false,
            scene: Rc::downgrade(scene),
            transformation: Transformation::new(),
            children: Vec::new(),
        }))
    }
}

impl Component for Group {
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
