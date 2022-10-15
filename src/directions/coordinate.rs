//! This module represents a point with an x and y coordinate.
//! We provide multiple helper function to modify coordinates
//! with other coordinates and the [`crate::direction::Vector`] module.
//!
//! # Sample usage
//!
//! ```
//! # use ws04::coordinate::Coordinate;
//! let position = Coordinate::new(3, 4);
//!
//! ```

use crate::directions::direction::Vector;
use std::convert::From;
use std::default::Default;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// Represent a 2D coordinate.
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Coordinate {
    /// x coordinate
    pub x: i32,
    /// y coordinate
    pub y: i32,
}

impl Coordinate {
    /// Create a new coordinate.
    ///
    /// # Arguments
    ///
    /// - `x` - The x coordinate of the new position.
    /// - `y` - The y coordinate of the new position.
    pub fn new(x: i32, y: i32) -> Coordinate {
        Coordinate { x, y }
    }

    fn y_in_range(&self, a: &Self, b: &Self) -> bool {
        if a.y <= b.y {
            a.y <= self.y && self.y <= b.y
        } else {
            b.y <= self.y && self.y <= a.y
        }
    }

    fn x_in_range(&self, a: &Self, b: &Self) -> bool {
        if a.x <= b.x {
            a.x <= self.x && self.x <= b.x && self.y_in_range(a, b)
        } else {
            b.x <= self.x && self.x <= a.x && self.y_in_range(a, b)
        }
    }

    /// Checks whether the point is within a rectangle box given by two
    /// diagonally opposite [`Coordinate`]s.
    ///
    /// # Arguments
    ///
    /// - `a` - One corner of the rectangular box.
    /// - `b` - The other corner of the rectangular box.
    ///
    /// # Examples
    ///
    /// ```
    /// use ws04::coordinate::Coordinate;
    /// let top_left = Coordinate::new(1, 1);
    /// let bottom_right = Coordinate::new(5, 5);
    ///
    /// assert!(Coordinate::new(1, 1).in_rectangle(&top_left, &bottom_right));
    /// assert!(Coordinate::new(1, 5).in_rectangle(&top_left, &bottom_right));
    /// assert!(Coordinate::new(3, 3).in_rectangle(&top_left, &bottom_right));
    /// assert!(Coordinate::new(5, 5).in_rectangle(&top_left, &bottom_right));
    /// ```
    ///
    /// The rectangle can be inferred even if you don't specify the top left and
    /// bottom right corners. E.g. we can still use the bottom left and top
    /// right corner:
    ///
    /// ```
    /// # use ws04::coordinate::Coordinate;
    /// let bottom_left = Coordinate::new(1, 5);
    /// let top_right = Coordinate::new(5, 1);
    ///
    /// assert!(Coordinate::new(1, 1).in_rectangle(&bottom_left, &top_right));
    /// assert!(Coordinate::new(1, 5).in_rectangle(&bottom_left, &top_right));
    /// assert!(Coordinate::new(3, 3).in_rectangle(&bottom_left, &top_right));
    /// assert!(Coordinate::new(5, 5).in_rectangle(&bottom_left, &top_right));
    /// ```
    pub fn in_rectangle(&self, a: &Self, b: &Self) -> bool {
        self.x_in_range(a, b)
    }

    /// Returns the magnitude of a vector between this Coordinate and another
    /// Coordinate.
    ///
    /// ```
    /// use ws04::coordinate::Coordinate;
    /// let c1 = Coordinate::new(4, 5);
    /// let c2 = Coordinate::new(1, 1);
    /// assert_eq!(c1.scalar_difference(&c2), 5f64);
    /// ```
    pub fn scalar_difference(&self, other: &Self) -> f64 {
        (self - other).magnitude()
    }
}

impl Default for Coordinate {
    /// Returns the default coordinate
    ///
    /// This has zero values for x and y
    fn default() -> Self {
        Coordinate { x: 0, y: 0 }
    }
}

impl Sub for &Coordinate {
    /// The resulting type after the `-` operator.
    type Output = Vector;

    /// Performs the `-` operation. See [`std::ops::Sub`]
    fn sub(self, rhs: &Coordinate) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub for Coordinate {
    /// The resulting type after the `-` operator.
    type Output = Vector;

    /// Performs the `-` operation. See [`std::ops::Sub`]
    fn sub(self, rhs: Coordinate) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add<Vector> for Coordinate {
    /// The resulting type after the `+` operator.
    type Output = Coordinate;

    /// Performs the `+` operation by adding a vector to a coordinate. See
    /// [`std::ops::Add`]
    fn add(self, rhs: Vector) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Add<&Vector> for Coordinate {
    /// The resulting type after the `+` operator.
    type Output = Coordinate;

    /// Performs the `+` operation by adding a vector to a coordinate. See
    /// [`std::ops::Add`]
    fn add(self, rhs: &Vector) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Vector> for Coordinate {
    /// Performs the `+=` operation by adding a vector to a coordinate. See
    /// [`std::ops::AddAssign`]
    fn add_assign(&mut self, rhs: Vector) {
        *self = Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<&Vector> for Coordinate {
    /// Performs the `+=` operation by adding a vector to a coordinate. See
    /// [`std::ops::AddAssign`]
    fn add_assign(&mut self, rhs: &Vector) {
        *self = Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub<Vector> for Coordinate {
    /// The resulting type after the `-` operator.
    type Output = Coordinate;

    /// Performs the `-` operation by subtracting a vector from a coordinate. See
    /// [`std::ops::Sub`]
    fn sub(self, rhs: Vector) -> Self::Output {
        Coordinate {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<&Vector> for Coordinate {
    /// The resulting type after the `-` operator.
    type Output = Coordinate;

    /// Performs the `-` operation by subtracting a vector from a coordinate. See
    /// [`std::ops::Sub`]
    fn sub(self, rhs: &Vector) -> Self::Output {
        Coordinate {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Vector> for Coordinate {
    /// Performs the `-=` operation by subtracting a vector from a coordinate. See
    /// [`std::ops::SubAssign`]
    fn sub_assign(&mut self, rhs: Vector) {
        *self = Coordinate {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<&Vector> for Coordinate {
    /// Performs the `-=` operation by subtracting a vector from a coordinate. See
    /// [`std::ops::SubAssign`]
    fn sub_assign(&mut self, rhs: &Vector) {
        *self = Coordinate {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl From<Vector> for Coordinate {
    /// Convert a vector to a coordinate
    ///
    /// This is the equivalent to adding the vector to a default coordinate
    fn from(v: Vector) -> Coordinate {
        Coordinate { x: v.x, y: v.y }
    }
}