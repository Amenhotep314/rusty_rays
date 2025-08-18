use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vector3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { x, y, z }
    }

    // Implementing vector-specific operations
    fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    fn cross(&self, other: &Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    fn normalized(&self) -> Self {
        let mag: f32 = self.magnitude();
        Self::new(self.x / mag, self.y / mag, self.z / mag)
    }
}

// Addition
macro_rules! impl_add_Vector3 {
    ($left:ty, $right:ty) => {
        impl ops::Add<$right> for $left {
            type Output = Vector3;

            fn add(self, other: $right) -> Self::Output {
                Vector3::new(self.x + other.x, self.y + other.y, self.z + other.z)
            }
        }
    }
}

impl_add_Vector3!(Vector3, Vector3);
impl_add_Vector3!(&Vector3, Vector3);
impl_add_Vector3!(Vector3, &Vector3);
impl_add_Vector3!(&Vector3, &Vector3);

// Subtraction
macro_rules! impl_sub_Vector3 {
    ($left:ty, $right:ty) => {
        impl ops::Sub<$right> for $left {
            type Output = Vector3;

            fn sub(self, other: $right) -> Self::Output {
                Vector3::new(self.x - other.x, self.y - other.y, self.z - other.z)
            }
        }
    }
}

impl_sub_Vector3!(Vector3, Vector3);
impl_sub_Vector3!(&Vector3, Vector3);
impl_sub_Vector3!(Vector3, &Vector3);
impl_sub_Vector3!(&Vector3, &Vector3);

// Element product
macro_rules! impl_mul_Vector3 {
    ($left:ty, $right:ty) => {
        impl ops::Mul<$right> for $left {
            type Output = Vector3;

            fn mul(self, other: $right) -> Self::Output {
                Vector3::new(self.x * other.x, self.y * other.y, self.z * other.z)
            }
        }
    }
}

impl_mul_Vector3!(Vector3, Vector3);
impl_mul_Vector3!(&Vector3, Vector3);
impl_mul_Vector3!(Vector3, &Vector3);
impl_mul_Vector3!(&Vector3, &Vector3);

// Element divide
macro_rules! impl_div_Vector3 {
    ($left:ty, $right:ty) => {
        impl ops::Div<$right> for $left {
            type Output = Vector3;

            fn div(self, other: $right) -> Self::Output {
                Vector3::new(self.x / other.x, self.y / other.y, self.z / other.z)
            }
        }
    }
}

impl_div_Vector3!(Vector3, Vector3);
impl_div_Vector3!(&Vector3, Vector3);
impl_div_Vector3!(Vector3, &Vector3);
impl_div_Vector3!(&Vector3, &Vector3);

// Scalar multiplication on right
impl ops::Mul<f32> for Vector3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self::Output {
        Self::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl ops::Mul<f32> for &Vector3 {
    type Output = Vector3;

    fn mul(self, scalar: f32) -> Self::Output {
        Vector3::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

// Scalar multiplication on the left
impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, vector: Vector3) -> Self::Output {
        Vector3::new(vector.x * self, vector.y * self, vector.z * self)
    }
}

impl ops::Mul<&Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, vector: &Vector3) -> Self::Output {
        Vector3::new(vector.x * self, vector.y * self, vector.z * self)
    }
}

// Scalar division
impl ops::Div<f32> for Vector3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Self::Output {
        if scalar == 0.0 {
            panic!("Vector3 division by zero.");
        }
        Vector3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

impl ops::Div<f32> for &Vector3 {
    type Output = Vector3;

    fn div(self, scalar: f32) -> Self::Output {
        if scalar == 0.0 {
            panic!("Vector3 division by zero.");
        }
        Vector3::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

// Negation
impl ops::Neg for Vector3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Neg for &Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3::new(-self.x, -self.y, -self.z)
    }
}
