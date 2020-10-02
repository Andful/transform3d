use cgmath::Vector3;
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use crate::{Component, Scene, Transformation};

pub struct Hitbox {
    hitbox_type: HitboxType,
    has_parent: bool,
    scene: Weak<RefCell<Scene>>,
    transformation: Transformation,
    children: Vec<Rc<RefCell<dyn Component>>>,
}

impl Hitbox {
    pub fn new(hitbox_type: HitboxType, scene: &Rc<RefCell<Scene>>) -> Hitbox {
        let result = Hitbox {
            hitbox_type,
            has_parent: false,
            scene: Rc::downgrade(scene),
            transformation: Transformation::new(),
            children: Vec::new(),
        };
        result
    }

    pub fn hits(&self, hitbox: &Hitbox) -> bool {
        match self.hitbox_type {
            HitboxType::SphereHitbox { center, radius } => {
                let c1 = center;
                let r1 = radius;
                match hitbox.hitbox_type {
                    HitboxType::SphereHitbox { center, radius } => {
                        Hitbox::sphere_sphere_hit(c1, r1, center, radius)
                    }
                    HitboxType::BoxHitbox { .. } => unimplemented!(),
                    HitboxType::PillHitbox { .. } => unimplemented!(),
                }
            }
            HitboxType::BoxHitbox { .. } => match hitbox.hitbox_type {
                HitboxType::SphereHitbox { .. } => unimplemented!(),
                HitboxType::BoxHitbox { .. } => unimplemented!(),
                HitboxType::PillHitbox { .. } => unimplemented!(),
            },
            HitboxType::PillHitbox { .. } => match hitbox.hitbox_type {
                HitboxType::SphereHitbox { .. } => unimplemented!(),
                HitboxType::BoxHitbox { .. } => unimplemented!(),
                HitboxType::PillHitbox { .. } => unimplemented!(),
            },
        }
    }

    pub fn sphere_sphere_hit(_c1: Vector3<f32>, _r1: f32, _c2: Vector3<f32>, _r2: f32) -> bool {
        unimplemented!();
    }

    pub fn sphere_box_hit(_c: Vector3<f32>, _r: f32, _max: Vector3<f32>, _min: Vector3<f32>) {
        unimplemented!();
    }

    pub fn sphere_pill_hit(
        _cc: Vector3<f32>,
        _cr: f32,
        _pc1: Vector3<f32>,
        _pc22: Vector3<f32>,
        _pr: f32,
    ) {
        unimplemented!();
    }

    pub fn box_pill_hit(
        _max: Vector3<f32>,
        _min: Vector3<f32>,
        _c1: Vector3<f32>,
        _c22: Vector3<f32>,
        _r: f32,
    ) {
        unimplemented!();
    }
}

pub enum HitboxType {
    SphereHitbox {
        center: Vector3<f32>,
        radius: f32,
    },
    BoxHitbox {
        max: Vector3<f32>,
        min: Vector3<f32>,
    },
    PillHitbox {
        x1: Vector3<f32>,
        x2: Vector3<f32>,
        radius: f32,
    },
}

impl Component for Hitbox {
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
