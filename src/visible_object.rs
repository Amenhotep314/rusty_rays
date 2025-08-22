use crate::material::Material;
use crate::ray::{Ray, RayResult};
use crate::vector3::Vector3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VisibleObject {
    Sphere {
        position: Vector3,
        radius: f32,
        material: Material,
    },
}

impl VisibleObject {
    pub fn test_intersect(&self, ray: Ray) -> RayResult {
        match self {
            VisibleObject::Sphere {
                position,
                radius,
                material,
            } => {
                let d: f32 = ray.direction.dot(&(ray.origin - position)).powi(2)
                    - (ray.origin - position).magnitude().powi(2)
                    - radius.powi(2);
                if d < 0.0 {
                    return RayResult::new_miss();
                }

                let offset: f32 = -ray.direction.dot(&(ray.origin - position));
                let t: f32 = offset - d.sqrt();

                if t < 0.0 {
                    let new_t: f32 = t - (2.0 * offset);
                    if new_t < 0.0 {
                        return RayResult::new_miss();
                    } // TODO : Implement complete refraction logic
                    return RayResult::new_miss();
                }

                let intersect: Vector3 = ray.origin + ray.direction * t;
                RayResult::new_hit(self.clone(), intersect)
            }
        }
    }
}
