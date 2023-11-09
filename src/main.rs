mod vector2;
mod vector3;
mod matrix4x4;
mod matrix;
mod vector4;

use crate::vector3::Vector3;
use std::fs::File;
use std::io::Write;

fn main() {
    let v3_1 = Vector3::new(1.0, 2.0, 3.0);
    let v3_2 = Vector3::new(4.0, 5.0, 6.0);
    let v3_3 = Vector3::new(0.0, 3.0, 0.0);
    let v3_4 = Vector3::new(5.0, 5.0, 0.0);
    let v3_5 = Vector3::new(4.0, 5.0, 1.0);
    let v3_6 = Vector3::new(4.0, 1.0, 3.0);

    let v3_7 = v3_5.cross(&v3_6);

    // Open the output.txt file for writing
    let mut file = File::create("output.txt").expect("Failed to create output.txt");

    // Write the text to the file instead of printing
    write!(file, "v1 + v2: {}\nv2 + v1: {}\n", v3_1.add(&v3_2), v3_2.add(&v3_1))
        .expect("Failed to write to output.txt");
    write!(file, "kąt pomiędzy [0,3,0], a [5,5,0]: {}\n", f64::to_degrees(v3_3.angle(&v3_4)))
        .expect("Failed to write to output.txt");
    write!(
        file,
        "wektor prostopadły do [4,5,1] i [4,1,3]: {}\n",
        v3_5.cross(&v3_6)
    )
        .expect("Failed to write to output.txt");
    write!(
        file,
        "znormalizowany wektor: {}\n",
        v3_7.normalise().unwrap()
    )
        .expect("Failed to write to output.txt");



    
}

