//! The vectors module contain

use std::ops;

#[derive(Debug)]
pub struct Vector {
    pub dimension: usize,
    pub coordinates: Vec<f64>,
}

impl Vector {

    /// Create a new Vector from a builtin Vec type array.
    /// A Vector is a representation of geometric vectors as we know,
    /// they are composed of coordinates and a dimension representing
    /// how many axis are involved.
    ///
    /// # Examples;
    ///
    /// ```
    /// use vecthrust::vectors::Vector;
    /// let vector = Vector::new(vec![1.0, 1.0]);
    /// assert_eq!(2, vector.dimension);
    /// ```
    pub fn new(coordinates: Vec<f64>) -> Vector {
        if coordinates.len() == 0 {
            panic!("Empty vectors are not allowed");
        }
        Vector{
            dimension: coordinates.len(),
            coordinates: coordinates,
        }
    }

    /// Format this vector as a string
    /// # Examples:
    ///
    /// ```
    /// use vecthrust::vectors::Vector;
    ///
    /// let vector = Vector::new(vec![1.0, 2.5099]);
    /// assert_eq!("Vector: [1, 2.5099]", vector.to_string());
    /// ```
    pub fn to_string(&self) -> String {
        return format!("Vector: {:?}", self.coordinates);
    }
}

impl PartialEq for Vector {

    /// Compares two vectors in order that:
    ///
    /// ```
    /// use vecthrust::vectors::Vector;
    ///
    /// let vector1 = Vector::new(vec![1.0, 2.5099]);
    /// let vector2 = Vector::new(vec![1.0, 2.5099]);
    /// let vector3 = Vector::new(vec![2.0, 2.5099]);
    /// assert!(vector1 == vector2);
    /// assert!(!(vector1 == vector3));
    /// ```
    fn eq(&self, other: &Vector) -> bool {
        return self.coordinates == other.coordinates;
    }
}

impl Eq for Vector {}

impl ops::Add<Vector> for Vector {

    type Output = Vector;

    /// Add two vectors creating a new one with the resulting coordinates.
    ///
    /// # Examples:
    ///
    /// ```
    /// use vecthrust::vectors::Vector;
    ///
    /// let vector1 = Vector::new(vec![1.0, 2.0]);
    /// let vector2 = Vector::new(vec![0.0, 4.0]);
    /// let result1 = Vector::new(vec![1.0, 6.0]);
    /// assert_eq!(result1, (vector1 + vector2));
    /// ```
    fn add(self, rhs: Vector) -> Vector {
        let mut result = vec![0.0; self.dimension];
        for i in 0..self.dimension {
            result[i] = self.coordinates[i] + rhs.coordinates[i];
        }

        return Vector::new(result);
    }
}

impl ops::Sub<Vector> for Vector {

    type Output = Vector;

    /// Subtract two vectors creating a new one with the resulting coordinates.
    ///
    /// # Examples:
    ///
    /// ```
    /// use vecthrust::vectors::Vector;
    ///
    /// let vector1 = Vector::new(vec![1.0, 2.0]);
    /// let vector2 = Vector::new(vec![0.0, 4.0]);
    /// let result1 = Vector::new(vec![1.0, -2.0]);
    /// assert_eq!(result1, (vector1 - vector2));
    /// ```
    fn sub(self, rhs: Vector) -> Vector {
        let mut result = vec![0.0; self.dimension];
        for i in 0..self.dimension {
            result[i] = self.coordinates[i] - rhs.coordinates[i];
        }

        return Vector::new(result);
    }
}

impl ops::Mul<f64> for Vector {
    type Output = Vector;

    /// Subtract two vectors creating a new one with the resulting coordinates.
    ///
    /// # Examples:
    ///
    /// ```
    /// use vecthrust::vectors::Vector;
    ///
    /// let vector = Vector::new(vec![1.0, 2.0]);
    /// let result = Vector::new(vec![2.0, 4.0]);
    /// assert_eq!(result, (vector * 2.0));
    /// ```
    fn mul(self, rhs: f64) -> Vector {
        let mut result = vec![0.0; self.dimension];
        for i in 0..self.dimension {
            result[i] = self.coordinates[i] * rhs;
        }

        return Vector::new(result);
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[should_panic(expected= "Empty vectors are not allowed")]
    fn test_create_with_empty_coordinates() {
        Vector::new(vec![]);
    }
}