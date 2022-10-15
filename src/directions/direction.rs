//! This module represents a vector with an x and y coordinate. These can be
//! added to our [`crate::coordinate::Coordinate`] values.

use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Debug, Eq, PartialEq, Clone)]
/// Struct representing a Vector of motion.
pub struct Vector {
    /// x direction
    pub x: i32,
    /// y direction
    pub y: i32,
}

#[derive(Debug, Eq, PartialEq)]
/// Enum describing the four cardinal directions.
pub enum CardinalDirection {
    North,
    East,
    South,
    West,
}

impl Vector {
    /// Create a new vector.
    ///
    /// ```
    /// # use ws04::direction::Vector;
    /// let v = Vector::new(3, 4);
    /// assert_eq!(v.x, 3);
    /// assert_eq!(v.y, 4);
    /// ```
    pub fn new(x: i32, y: i32) -> Vector {
        Vector { x, y }
    }

    /// Returns the scalar magnitude of this vector
    ///
    /// ```
    /// # use ws04::direction::Vector;
    /// let v = Vector::new(3, 4);
    /// assert_eq!(v.magnitude(), 5f64);
    /// ```
    pub fn magnitude(&self) -> f64 {
        (f64::from(self.x).powi(2) + f64::from(self.y).powi(2)).sqrt()
    }
}

impl Add for Vector {
    /// The resulting type after the `+` operator.
    type Output = Vector;

    /// Performs the `+` operation. See [`std::ops::Add`]
    fn add(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add for &Vector {
    /// The resulting type after the `+` operator.
    type Output = Vector;

    /// Performs the `+` operation. See [`std::ops::Add`]
    fn add(self, rhs: &Vector) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vector {
    /// Performs the `+=` operation. See [`std::ops::AddAssign`]
    fn add_assign(&mut self, rhs: Vector) {
        *self = Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vector {
    /// The resulting type after the `-` operator.
    type Output = Vector;

    /// Performs the `-` operation. See [`std::ops::Sub`]
    fn sub(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub for &Vector {
    /// The resulting type after the `-` operator.
    type Output = Vector;

    /// Performs the `-` operation. See [`std::ops::Sub`]
    fn sub(self, rhs: &Vector) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Vector {
    /// Performs the `-=` operation. See [`std::ops::SubAssign`]
    fn sub_assign(&mut self, rhs: Vector) {
        *self = Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Mul<i32> for Vector {
    /// The resulting type after the `*` operator.
    type Output = Vector;

    /// Performs the `*` operation. See [`std::ops::Mul`]
    fn mul(self, rhs: i32) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign<i32> for Vector {
    /// Performs the `*=` operation. See [`std::ops::MulAssign`]
    fn mul_assign(&mut self, rhs: i32) {
        *self = Vector {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}