mod vector3;

use vector3::{Ray, RayHit, Vector3};

fn main() {
    let origin: Vector3 = Vector3::new(0.0, 0.0, 0.0);
    let direction: Vector3 = Vector3::new(1.0, 1.0, 1.0);
    let ray: Ray = Ray::new(origin, direction.normalized());
    println!("Ray: {}", ray);
}
