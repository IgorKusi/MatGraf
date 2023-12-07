use crate::quaterion::Quaternion;

mod vector2;
mod vector3;
mod matrix;
mod vector4;
mod quaterion;

use std::fs::File;
use std::io::{self, Write};
use crate::vector3::Vector3;

fn main() {
    // 1. Utwórz kwaterniony
    let quaternion1 = Quaternion::new(1.0, 2.0, 3.0, 4.0);
    let quaternion2 = Quaternion::new(5.0, 6.0, 7.0, 8.0);

    // 2. Sprawdź operacje
    let sum = quaternion1 + quaternion2;
    let difference = quaternion1 - quaternion2;
    let product = quaternion1 * quaternion2;

    // 3. Obróć punkt [-1, -1, -1] o 270° wokół osi X
    let point = Quaternion::new(0.0, -1.0, -1.0, -1.0);
    let angle_degrees: f64 = 270.0;
    let axis: Vector3 = Vector3::new(1.0, 0.0, 0.0);

    //rotate point around axis x by 270 degrees
    let rotated_point = Quaternion::rotate(point.v, angle_degrees, axis);

    // 4. Udowodnij brak przemienności mnożenia kwaternionów
    let product1 = quaternion1 * quaternion2;
    let product2 = quaternion2 * quaternion1;

    // Wyniki zapisz do pliku "wyniki.txt"
    let mut file = File::create("wyniki.txt").expect("Unable to create file");

    writeln!(&mut file, "Quaternion Operations:").expect("Unable to write to file");

    writeln!(&mut file, "Quaternion 1: {:?}", quaternion1).expect("Unable to write to file");
    writeln!(&mut file, "Quaternion 2: {:?}", quaternion2).expect("Unable to write to file");
    writeln!(&mut file, "Sum: {:?}", sum).expect("Unable to write to file");
    writeln!(&mut file, "Difference: {:?}", difference).expect("Unable to write to file");
    writeln!(&mut file, "Product: {:?}", product).expect("Unable to write to file");

    writeln!(&mut file, "\nRotate Point [-1, -1, -1] by 270 degrees around X:").expect("Unable to write to file");
    writeln!(&mut file, "Original Point: {:?}", point).expect("Unable to write to file");
    writeln!(&mut file, "Rotated Point: {:?}", rotated_point).expect("Unable to write to file");

    writeln!(&mut file, "\nDemonstrate Lack of Quaternion Multiplication Commutativity:").expect("Unable to write to file");
    writeln!(&mut file, "Quaternion 1 * Quaternion 2: {:?}", product1).expect("Unable to write to file");
    writeln!(&mut file, "Quaternion 2 * Quaternion 1: {:?}", product2).expect("Unable to write to file");
}
