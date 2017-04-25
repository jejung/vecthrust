//! The vecthrust crate includes functionality for working with linear algebra.

pub struct Vector<'a> {
    coordinates: &'a Vec<f64>,
    dimension: usize,
}

impl<'a> Vector<'a> {
    fn new(coordinates: &'a Vec<f64>) -> Vector {
        Vector{
            coordinates: coordinates,
            dimension: coordinates.len()
        }
    } 
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_dimension_syncronism() {
         let values = vec![1.0, 1.0]; 
         let vector = super::Vector::new(&values);
         assert_eq!(2, vector.dimension);
    }
}
