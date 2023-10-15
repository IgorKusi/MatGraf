mod vector2;
mod vector3;

use crate::vector3::Vector3;

fn main() {
    let v3_1 = Vector3::new(1.0,2.0,3.0);
    let v3_2 = Vector3::new(4.0, 5.0, 6.0);
    let v3_3 = Vector3::new(0.0, 3.0, 0.0);
    let v3_4 = Vector3::new(5.0, 5.0, 0.0);
    let v3_5 = Vector3::new(4.0,5.0,1.0);
    let v3_6 = Vector3::new(4.0, 1.0, 3.0);

    let v3_7 = v3_5.cross(&v3_6);

    println!("v1 +v2: {} \nv2 + v1: {}", v3_1.add(&v3_2), v3_2.add(&v3_1));
    println!("kąt pomiędzy [0,3,0], a [5,5,0]: {}[rad]", v3_3.angle(&v3_4));
    println!("wektor prostopadły do [4,5,1] i [4,1,3]: {}", v3_5.cross(&v3_6));
    println!("znormalozowany wektor: {}", v3_7.normalise());
}
