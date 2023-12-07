/// Add two arrays arrays together elementwise
/// and return the result
pub fn add_arrays(array1: [f64; 2], array2: [f64; 2]) -> [f64; 2] {
    [array1[0] + array2[0], array1[1] + array2[1]]
}

/// Allows for scalar multiplication with an array
/// that captures either position or velocity
pub fn scalar_mult(array: [f64; 2], scalar: f64) -> [f64; 2] {
    let mut result = [0.0, 0.0];
        for (i, &element) in array.iter().enumerate() {
        result[i] = element * scalar; 
    }
    result
}