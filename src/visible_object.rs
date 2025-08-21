use crate::material::Material;
use crate::vector3::Vector3;

pub enum VisibleObject {
    Sphere {
        position: Vector3,
        radius: f64,
        material: Material,
    },
}
