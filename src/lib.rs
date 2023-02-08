use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct CapacityExhaustedError {
    error: String,
}

impl fmt::Display for CapacityExhaustedError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.error)
    }
}
impl Error for CapacityExhaustedError {}
impl CapacityExhaustedError {
    fn new(error: String) -> Self {
        CapacityExhaustedError { error }
    }
}

struct Matrix {
    matrix: Vec<i128>,
    y: usize,
    x: usize,
    capacity: usize,
}

impl Matrix {
    fn new(x: usize, y: usize) -> Self {
        Matrix {
            x,
            y,
            matrix: Vec::new(),
            capacity: x + y,
        }
    }

    fn add_value(&mut self, i: i128) -> Result<usize, CapacityExhaustedError> {
        if self.matrix.len() < self.capacity {
            self.matrix.push(i);
            Ok(self.capacity - self.matrix.len())
        } else {
            Err(CapacityExhaustedError {
                error: format!("Capacity of {} exhausted.", self.capacity),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_matrix() {
        let mut matrix = Matrix::new(2, 2);
        for i in 0..4 {
            match matrix.add_value(i) {
                Ok(i) => assert!(i > usize::MIN || i == usize::MIN),
                Err(_) => panic!("Must not run into an error."),
            }
        }
        match matrix.add_value(5) {
            Ok(_) => panic!("Must run into capacity error now."),
            Err(e) => assert!(format!("{e}") == "Capacity of 4 exhausted."),
        }
        //assert_eq!(result, 4);
    }
}
