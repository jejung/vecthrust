//! The vecthrust crate includes functionality for working with linear algebra.

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
    /// use vecthrust::Vector;
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
    /// use vecthrust::Vector;
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
    /// use vecthrust::Vector;
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

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[should_panic(expected= "Empty vectors are not allowed")]
    fn test_create_with_empty_coordinates() {
        let vector = Vector::new(vec![]);
    }
}
