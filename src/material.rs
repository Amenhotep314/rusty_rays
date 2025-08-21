pub struct Material {
    pub color: [f32; 3],
    pub roughness: f32,
    pub opacity: f32,
    pub refractive_index: f32,
    pub is_light: bool,
}

impl Material {
    pub fn new(color: [f32; 3], roughness: f32, opacity: f32, refractive_index: f32) -> Material {
        let is_light: bool = color[0] > 1.0 || color[1] > 1.0 || color[2] > 1.0;
        Material {
            color,
            roughness,
            opacity,
            refractive_index,
            is_light,
        }
    }
}
