mod vector2;
mod vector3;
mod matrix;
mod vector4;

use crate::vector3::Vector3;
use crate::vector4::Vector4;
use std::fs::File;
use std::io::Write;
use crate::matrix::Matrix4;


fn main() {
    // Test identity function
    let identity_matrix = Matrix4::identity();
    println!("Identity Matrix:\n{:?}", identity_matrix);

    // Test translate function
    let translation_vector = Vector3::new(1.0, 2.0, 3.0);
    let translation_matrix = Matrix4::translate(&translation_vector);
    println!("Translation Matrix:\n{:?}", translation_matrix);

    // Test scale function
    let scale_vector = Vector3::new(2.0, 2.0, 2.0);
    let scale_matrix = Matrix4::scale(&scale_vector);
    println!("Scale Matrix:\n{:?}", scale_matrix);

    // Test rot_x function
    let rotation_matrix_x = Matrix4::rot_x(std::f64::consts::FRAC_PI_4);
    println!("Rotation Matrix X:\n{:?}", rotation_matrix_x);

    // Test rot_y function
    let rotation_matrix_y = Matrix4::rot_y(std::f64::consts::FRAC_PI_4);
    println!("Rotation Matrix Y:\n{:?}", rotation_matrix_y);

    // Test rot_z function
    let rotation_matrix_z = Matrix4::rot_z(std::f64::consts::FRAC_PI_4);
    println!("Rotation Matrix Z:\n{:?}", rotation_matrix_z);

    // Test det function
    let det_result = identity_matrix.det();
    println!("Determinant of Identity Matrix: {}", det_result);

    // Test col_as_vec4 function
    let col_vector = translation_matrix.col_as_vec4(1);
    println!("Column as Vector4:\n{:?}", col_vector);

    // Test row_as_vec4 function
    let row_vector = translation_matrix.row_as_vec4(2);
    println!("Row as Vector4:\n{:?}", row_vector);

    // Test add function
    let sum_matrix = identity_matrix.add(&translation_matrix);
    println!("Sum Matrix:\n{:?}", sum_matrix);

    // Test mut_add function (in-place addition)
    let mut mut_sum_matrix = identity_matrix;
    mut_sum_matrix.mut_add(&translation_matrix);
    println!("In-Place Sum Matrix:\n{:?}", mut_sum_matrix);

    // Test sub function
    let diff_matrix = identity_matrix.sub(&translation_matrix);
    println!("Difference Matrix:\n{:?}", diff_matrix);

    // Test mut_sub function (in-place subtraction)
    let mut mut_diff_matrix = identity_matrix;
    mut_diff_matrix.mut_sub(&translation_matrix);
    println!("In-Place Difference Matrix:\n{:?}", mut_diff_matrix);

    // Test mul_sclr function
    let scalar = 2.0;
    let scaled_matrix = identity_matrix.mul_sclr(scalar);
    println!("Scaled Matrix:\n{:?}", scaled_matrix);

    // Test mut_mul_sclr function (in-place scalar multiplication)
    let mut mut_scaled_matrix = identity_matrix;
    mut_scaled_matrix.mut_mul_sclr(scalar);
    println!("In-Place Scaled Matrix:\n{:?}", mut_scaled_matrix);

    // Test mul function
    let product_matrix = identity_matrix.mul(&translation_matrix);
    println!("Product Matrix:\n{:?}", product_matrix);

    // Test mut_mul function (in-place matrix multiplication)
    let mut mut_product_matrix = identity_matrix;
    mut_product_matrix.mut_mul(&translation_matrix);
    println!("In-Place Product Matrix:\n{:?}", mut_product_matrix);

    // Test mul_vec function
    let vector4 = Vector4::new(1.0, 2.0, 3.0, 1.0);
    let transformed_vector = translation_matrix.mul_vec(&vector4);
    println!("Transformed Vector:\n{:?}", transformed_vector);

    // Test transpose function
    let transposed_matrix = translation_matrix.transpose();
    println!("Transposed Matrix:\n{:?}", transposed_matrix);

    // Test mut_transpose function (in-place matrix transpose)
    let mut mut_transposed_matrix = translation_matrix;
    mut_transposed_matrix.mut_transpose();
    println!("In-Place Transposed Matrix:\n{:?}", mut_transposed_matrix);

    // Test invert function
    let inverted_matrix = translation_matrix.invert();
    println!("Inverted Matrix:\n{:?}", inverted_matrix);

    // Test mut_invert function (in-place matrix inversion)
    let mut mut_inverted_matrix = translation_matrix;
    mut_inverted_matrix.mut_invert();
    println!("In-Place Inverted Matrix:\n{:?}", mut_inverted_matrix);

    // Test operator overloading

    // Test addition using '+' operator
    let sum_op_matrix = identity_matrix + &translation_matrix;
    println!("Sum Matrix (using '+ operator'):\n{:?}", sum_op_matrix);

    // Test in-place addition using '+=' operator
    let mut mut_sum_op_matrix = identity_matrix;
    mut_sum_op_matrix += &translation_matrix;
    println!("In-Place Sum Matrix (using '+= operator'):\n{:?}", mut_sum_op_matrix);

    // Test subtraction using '-' operator
    let diff_op_matrix = identity_matrix - &translation_matrix;
    println!("Difference Matrix (using '- operator'):\n{:?}", diff_op_matrix);

    // Test in-place subtraction using '-=' operator
    let mut mut_diff_op_matrix = identity_matrix;
    mut_diff_op_matrix -= &translation_matrix;
    println!("In-Place Difference Matrix (using '-= operator'):\n{:?}", mut_diff_op_matrix);

    // Test scalar multiplication using '*' operator
    let scalar_op_matrix = identity_matrix * 2.0;
    println!("Scaled Matrix (using '* operator'):\n{:?}", scalar_op_matrix);

    // Test in-place scalar multiplication using '*=' operator
    let mut mut_scalar_op_matrix = identity_matrix;
    mut_scalar_op_matrix *= 2.0;
    println!("In-Place Scaled Matrix (using '*= operator'):\n{:?}", mut_scalar_op_matrix);

    // Test matrix multiplication using '*' operator
    let product_op_matrix = identity_matrix * &translation_matrix;
    println!("Product Matrix (using '* operator'):\n{:?}", product_op_matrix);

    // Test in-place matrix multiplication using '*=' operator
    let mut mut_product_op_matrix = identity_matrix;
    mut_product_op_matrix *= &translation_matrix;
    println!("In-Place Product Matrix (using '*= operator'):\n{:?}", mut_product_op_matrix);

    // Test negation using '-' operator
    let negated_matrix = -identity_matrix;
    println!("Negated Matrix (using '- operator'):\n{:?}", negated_matrix);
}
