pub trait Vector: Sized {
    fn x(&self) -> f32;

    fn y(&self) -> f32;

    fn len(&self) -> f32 {
        (self.x() * self.x() + self.y() * self.y()).sqrt()
    }
}

impl<'a, T: Vector> Vector for &'a T {
    fn x(&self) -> f32 {
        self.x()
    }

    fn y(&self) -> f32 {
        self.y()
    }
}

macro_rules! impl_vector {
    ($n:tt) => {
        #[derive(Component, Clone, Copy, Debug, Serialize, Deserialize)]
        pub struct $n {
            pub x: f32,
            pub y: f32,
        }

        impl $n {
            pub fn new(x: f32, y: f32) -> Self {
                Self { x, y }
            }

            pub fn zero() -> Self {
                Self::new(0.0, 0.0)
            }

            pub fn xcomp(&self) -> Self {
                if self.x == 0.0 {
                    Self::zero()
                } else {
                    Self::new(self.x / self.x.abs(), 0.0)
                }
            }

            pub fn ycomp(&self) -> Self {
                if self.y == 0.0 {
                    Self::zero()
                } else {
                    Self::new(0.0, self.y / self.y.abs())
                }
            }

            pub fn min(&self, vec: &Self) -> Self {
                if self.len() < vec.len() {
                    *self
                } else {
                    *vec
                }
            }

            pub fn max(&self, vec: &Self) -> Self {
                if self.len() > vec.len() {
                    *self
                } else {
                    *vec
                }
            }

            pub fn to_vec(&self) -> ncollide2d::math::Vector<f32> {
                use ncollide2d::math::Vector;
                Vector::new(self.x, self.y)
            }
        }

        impl From<ncollide2d::math::Vector<f32>> for $n {
            fn from(v: ncollide2d::math::Vector<f32>) -> Self {
                Self::new(v[0], v[1])
            }
        }

        impl Vector for $n {
            fn x(&self) -> f32 {
                self.x
            }

            fn y(&self) -> f32 {
                self.y
            }
        }

        impl std::ops::Neg for $n {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self::new(-self.x(), -self.y())
            }
        }

        impl<T: Vector> std::ops::Add<T> for $n {
            type Output = Self;

            fn add(self, rhs: T) -> Self::Output {
                Self::new(self.x() + rhs.x(), self.y() + rhs.y())
            }
        }

        impl<T: Vector> std::ops::AddAssign<T> for $n {
            fn add_assign(&mut self, rhs: T) -> () {
                *self = *self + rhs;
            }
        }

        impl<T: Vector> std::ops::Sub<T> for $n {
            type Output = Self;

            fn sub(self, rhs: T) -> Self::Output {
                Self::new(self.x() - rhs.x(), self.y() - rhs.y())
            }
        }

        impl<T: Vector> std::ops::SubAssign<T> for $n {
            fn sub_assign(&mut self, rhs: T) -> () {
                *self = *self - rhs;
            }
        }

        impl std::ops::Mul<f32> for $n {
            type Output = Self;

            fn mul(self, rhs: f32) -> Self::Output {
                Self::new(self.x() * rhs, self.y() * rhs)
            }
        }

        impl std::ops::MulAssign<f32> for $n {
            fn mul_assign(&mut self, rhs: f32) -> () {
                *self = *self * rhs;
            }
        }

        impl std::ops::Div<f32> for $n {
            type Output = Self;

            fn div(self, rhs: f32) -> Self::Output {
                Self::new(self.x() / rhs, self.y() / rhs)
            }
        }

        impl std::ops::DivAssign<f32> for $n {
            fn div_assign(&mut self, rhs: f32) -> () {
                *self = *self / rhs;
            }
        }

        impl std::cmp::PartialEq for $n {
            fn eq(&self, other: &$n) -> bool {
                let d = *self - *other;
                d.x() < 0.001 && d.y() < 0.001
            }
        }

        impl std::cmp::Eq for $n {}
    };
}
