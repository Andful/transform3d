use cgmath::Euler;
use cgmath::Matrix4;
use cgmath::Quaternion;
use cgmath::Rad;
use cgmath::SquareMatrix;
use cgmath::Vector3;

#[derive(Debug)]
pub struct Transformation {
    rooted: bool,
    needs_update: bool,
    translation: Vector3<f32>,
    rotation: Quaternion<f32>,
    scaling: Vector3<f32>,
    local_transformation: Matrix4<f32>,
    global_transformation: Matrix4<f32>,
}

impl Transformation {
    pub fn new() -> Transformation {
        Transformation {
            rooted: false,
            needs_update: false,
            translation: Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::new(1.0, 0.0, 0.0, 0.0),
            scaling: Vector3::new(1.0, 1.0, 1.0),
            local_transformation: SquareMatrix::identity(),
            global_transformation: SquareMatrix::identity(),
        }
    }

    pub fn rotate_x(&mut self, x_rotation: f32) {
        self.rotation = self.rotation
            * Quaternion::<f32>::from(Euler {
                x: Rad(x_rotation),
                y: Rad(0.0),
                z: Rad(0.0),
            });
        self.change_made();
    }

    pub fn rotate_y(&mut self, y_rotation: f32) {
        self.rotation = self.rotation
            * Quaternion::<f32>::from(Euler {
                x: Rad(0.0),
                y: Rad(y_rotation),
                z: Rad(0.0),
            });
        self.change_made();
    }

    pub fn rotate_z(&mut self, z_rotation: f32) {
        self.rotation = self.rotation
            * Quaternion::<f32>::from(Euler {
                x: Rad(0.0),
                y: Rad(0.0),
                z: Rad(z_rotation),
            });
        self.change_made();
    }

    pub fn rotate(&mut self, rotation: &Quaternion<f32>) {
        self.rotation = self.rotation * rotation;
        self.change_made();
    }

    pub fn translate(&mut self, additional_translation: &Vector3<f32>) {
        self.translation += *additional_translation;
        self.change_made();
    }

    pub fn scale(&mut self, additional_scale: &Vector3<f32>) {
        self.scaling += *additional_scale;
        self.change_made();
    }

    pub fn set_rotation(&mut self, rotation: &Quaternion<f32>) {
        self.rotation = *rotation;
        self.change_made();
    }

    pub fn set_translation(&mut self, translation: &Vector3<f32>) {
        self.translation = *translation;
        self.change_made();
    }

    pub fn set_scaling(&mut self, scaling: &Vector3<f32>) {
        self.scaling = *scaling;
        self.change_made();
    }

    pub fn update_local(&mut self, forced: bool) {
        if self.needs_update || forced {
            self.local_transformation = Matrix4::from_translation(self.translation)
                * Matrix4::<f32>::from(self.rotation)
                * Matrix4::from_nonuniform_scale(self.scaling.x, self.scaling.y, self.scaling.z);
        }
        self.needs_update = false;
    }

    fn change_made(&mut self) {
        self.needs_update = true;
    }

    pub fn update_global(&mut self, parent_transform: &Matrix4<f32>) {
        self.global_transformation = self.local_transformation * parent_transform;
    }

    pub fn global_transformation(&self) -> &Matrix4<f32> {
        &self.global_transformation
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use cgmath::Vector4;

    #[test]
    fn test_add() {
        let mut t = Transformation::new();
        let point = Vector4::<f32>::new(1.0, 0.0, 0.0, 1.0);

        t.rotate_y(std::f32::consts::PI);
        t.update_local(false);

        let rot_point = t.local_transformation * point;
        println!("{:?}", t.local_transformation);
        assert!((rot_point.x + 1.0).abs() < 1e-6);
        assert!(rot_point.y.abs() < 1e-6);
        assert!(rot_point.z.abs() < 1e-6);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        //assert_eq!(bad_add(1, 2), 3);
    }
}
