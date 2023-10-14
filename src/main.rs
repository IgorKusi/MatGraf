mod vector;

use vector::Vector2;

fn main() {
    println!("Hello, world!");
    let v1 = Vector2::new(21.0, 20.0);
    let v2 = Vector2::new(10.0, -10.0);
    let scalar: f64 = 10.0;

    let sum = v1.add(&v2);
    let sub = v2.sub(&v1);
    let scalmar = v1.scal_mul(&scalar);

    println!("SUM");
    println!("{}", sum);
    println!("\nSUB");
    println!("{}", sub);
    println!("\nSCALAR MULTIPLIKEJSZYN");
    println!("{}", scalmar)

}
