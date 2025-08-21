mod material;
mod ray;
mod vector3;
mod visible_object;

use ray::{Ray, RayResult};
use vector3::Vector3;

fn main() {
    let origin: Vector3 = Vector3::new(0.0, 0.0, 0.0);
    let direction: Vector3 = Vector3::new(1.0, 1.0, 1.0);
    let ray: Ray = Ray::new(origin, direction.normalized());
    println!("Ray: {}", ray);
}
