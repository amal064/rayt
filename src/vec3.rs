use std::ops;

#[derive(Copy, Clone, Default, Debug)]
pub struct Vec3(pub f32, pub f32, pub f32);

impl Vec3 {
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3(
            self.1 * rhs.2 - self.2 * rhs.1,
            -(self.0 * rhs.2 - self.2 * rhs.0),
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.dot(self))
    }

    pub fn into_unit(self) -> Self {
        self / self.length()
    }

    pub fn to_rgb(self) -> [u8; 3] {
        fn f(num: f32) -> u8 {
            if num < 0.0 {
                0
            } else if num >= 1.0 {
                255
            } else {
                (num * 255.99) as u8
            }
        }
        [f(self.0), f(self.1), f(self.2)]
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        Vec3(self.0 / rhs, self.1 / rhs, self.2 / rhs)
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

pub enum Axis {
    X,
    Y,
    Z,
}
impl ops::Index<Axis> for Vec3 {
    type Output = f32;

    fn index(&self, idx: Axis) -> &Self::Output {
        use Axis::{X, Y, Z};
        match idx {
            X => &self.0,
            Y => &self.1,
            Z => &self.2,
        }
    }
}
