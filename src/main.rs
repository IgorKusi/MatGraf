mod vector2;
mod vector3;
mod matrix;
mod vector4;

use crate::vector3::Vector3;
use crate::vector4::Vector4;
use crate::matrix::Matrix4;


fn main() {
    // Test identity function
    println!("Identity Matrix:\n{:?}", Matrix4::identity());

    // Test translate, scale
    let vector = Vector3::new(5.0, 4.0, 3.0);
    let matrix = Matrix4::translate(&vector);
    println!("Translation Matrix:\n{:?}", matrix);
    println!("Scale Matrix:\n{:?}", Matrix4::scale(&vector));

    // Test rotation
    println!("Rotation Matrix X:\n{:?}", Matrix4::rot_x(std::f64::consts::FRAC_PI_2));
    println!("Rotation Matrix Y:\n{:?}", Matrix4::rot_y(std::f64::consts::FRAC_PI_2));
    println!("Rotation Matrix Z:\n{:?}", Matrix4::rot_z(std::f64::consts::FRAC_PI_2));

    // Test vector pick
    println!("Column as Vector4:\n{:?}", matrix.col_as_vec4(0));
    println!("Row as Vector4:\n{:?}", matrix.row_as_vec4(0));

    // Test determinant
    println!("Determinant of Scale Matrix: {}", Matrix4::scale(&vector).det());

    // Test addition
    println!("Sum of Translation and Scale:\n{:?}", matrix.add_mtrx(&Matrix4::scale(&vector)));

    let mut mut_sum_matrix = matrix;
    mut_sum_matrix.mut_add_mtrx(&Matrix4::scale(&vector));
    println!("In-Place Sum Matrix:\n{:?}", mut_sum_matrix);

    // Test subtraction
    println!("Difference Matrix of Translate and Scale:\n{:?}", matrix.sub_mtrx(&Matrix4::scale(&vector)));

    let mut mut_diff_matrix = matrix;
    mut_diff_matrix.mut_sub_mtrx(&Matrix4::scale(&vector));
    println!("In-Place Difference Matrix:\n{:?}", mut_diff_matrix);

    // Test scalar multiplication
    let scalar = 2.0;
    println!("Scaled Matrix:\n{:?}", matrix.mul_sclr(scalar));

    let mut mut_scaled_matrix = matrix;
    mut_scaled_matrix.mut_mul_sclr(scalar);
    println!("In-Place Scaled Matrix:\n{:?}", mut_scaled_matrix);

    // Test multiplication
    println!("Product Matrix:\n{:?}", matrix.mul_mtrx(&Matrix4::scale(&vector)));

    let mut mut_product_matrix = matrix;
    mut_product_matrix.mut_mul_mtrx(&Matrix4::scale(&vector));
    println!("In-Place Product Matrix:\n{:?}", mut_product_matrix);

    // Test vector4 multiplication
    println!("Transformed Vector:\n{:?}", matrix.mul_vec(&Vector4::new(5.0, 4.0, 3.0, 5.0)));

    // Test transposition
    println!("Transposed Matrix:\n{:?}", matrix.transpose());

    let mut mut_transposed_matrix = matrix;
    mut_transposed_matrix.mut_transpose();
    println!("In-Place Transposed Matrix:\n{:?}", mut_transposed_matrix);

    // Test inversion
    println!("Inverted Scale + Translate Matrix:\n{:?}", (Matrix4::scale(&vector)+&Matrix4::translate(&vector)).invert());

    let mut mut_inverted_matrix = Matrix4::scale(&vector) + &Matrix4::translate(&vector);
    mut_inverted_matrix.mut_invert();
    println!("In-Place Inverted Matrix:\n{:?}", mut_inverted_matrix);

    // Test operator overloading

    // Test addition using '+' operator
    println!("Sum Matrix (using '+ operator'):\n{:?}", matrix + &Matrix4::scale(&vector));

    let mut mut_sum_op_matrix = matrix;
    mut_sum_op_matrix += &Matrix4::scale(&vector);
    println!("In-Place Sum Matrix (using '+= operator'):\n{:?}", mut_sum_op_matrix);

    // Test subtraction using '-' operator
    println!("Difference Matrix (using '- operator'):\n{:?}", matrix - &Matrix4::scale(&vector));

    let mut mut_diff_op_matrix = matrix;
    mut_diff_op_matrix -= &Matrix4::scale(&vector);
    println!("In-Place Difference Matrix (using '-= operator'):\n{:?}", mut_diff_op_matrix);

    // Test scalar multiplication using '*' operator
    println!("Scaled Matrix (using '* operator'):\n{:?}", matrix * 2.0);

    let mut mut_scalar_op_matrix = matrix;
    mut_scalar_op_matrix *= 2.0;
    println!("In-Place Scaled Matrix (using '*= operator'):\n{:?}", mut_scalar_op_matrix);

    // Test matrix multiplication using '*' operator
    println!("Product Matrix (using '* operator'):\n{:?}", matrix * &Matrix4::scale(&vector));

    let mut mut_product_op_matrix = matrix;
    mut_product_op_matrix *= &Matrix4::scale(&vector);
    println!("In-Place Product Matrix (using '*= operator'):\n{:?}", mut_product_op_matrix);

    // Test negation using '-' operator
    println!("Negated Matrix (using '- operator'):\n{:?}", -(Matrix4::scale(&vector) + &Matrix4::translate(&vector)));

    println!("Rotated vector: \n{:?}", Matrix4::rot_y(std::f64::consts::FRAC_PI_4) * &Vector4::new(1.0, 0.0, 0.0, 1.0));
    println!("Translate * scale: \n{:?}", Matrix4::translate(&vector) * &Matrix4::scale(&vector));
    println!("Scale * translate: \n{:?}", Matrix4::scale(&vector) * &Matrix4::translate(&vector));
}
