use crate::vector3::Vector3;
use crate::visible_object::VisibleObject;
use std::fmt;

// Ray Type
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Ray {
            origin,
            direction: direction.normalized(),
        }
    }
}

// To String
impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} + t * {}", self.origin, self.direction)
    }
}

// RayHit Type
pub enum RayResult {
    RayHit {
        target: VisibleObject,
        point: Vector3,
    },
    RayMiss,
}

impl RayResult {
    fn new_hit(target: VisibleObject, point: Vector3) -> Self {
        RayResult::RayHit { target, point }
    }

    fn new_miss() -> Self {
        RayResult::RayMiss {}
    }
}
