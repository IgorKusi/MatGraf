use crate::quaterion::Quaternion;

mod vector2;
mod vector3;
mod matrix;
mod vector4;
mod quaterion;
mod line2D;
mod line3D;
mod plane;
mod sphere;

use std::fs::File;
use std::io::{self, Write};
use crate::plane::Plane;
use crate::vector3::Vector3;

fn main() {
    //linia 1 przechodząca przez punkt (1,6,5) i (4,7,10)
    let l1 = line3D::Line3D::new_p((1.0, 6.0, 5.0), (4.0, 7.0, 10.0));
    //wyswietl linie
    println!("Linia 1: {:?}", l1.display_line());

    //linia 2 przechodząca przez punkt (0,-11,6) i (1,-16,9)
    let l2 = line3D::Line3D::new_p((0.0, -11.0, 6.0), (1.0, -16.0, 9.0));
    //wyswietl linie
    println!("Linia 2: {:?}", l2.display_line());

    //wyswietl czy linie sie przecinaja
    println!("Czy linie sie przecinaja: {:?}", l1.do_lines_intersect(&l2));

    //wyswietl punk przecięcia linii
    println!("Punkt przecięcia linii: {:?}", l1.calculate_point_of_intersection(&l2).unwrap());

    //wyswietl kat pomiedzy liniami
    println!("Kat pomiedzy liniami: {:?}", l1.calculate_angle_between_lines(&l2));

    println!();

    //plaszczyzna 2x+3y+3z-8=0
    let p1 = plane::Plane::new_p((0.0,0.0,8.0/3.0), (1.0,1.0,1.0), (1.0,2.0,2.0/3.0));
    //wyswietl plaszczyzne
    println!("Plaszczyzna: {:?}", p1);

    //linia przechodzaca przez punkt (-2,2,-1) i (1,1,1)
    let l3 = line3D::Line3D::new_p((-2.0, 2.0, -1.0), (1.0, 1.0, 1.0));
    //wyswietl linie
    println!("Linia: {:?}", l3.display_line());

    //wyswietl czy linia i plaszczyzna sie przecinaja
    println!("Czy linia i plaszczyzna sie przecinaja: {:?}", l3.does_intersect_with_plane(&p1));

    //wyswietl punkt przeciecia linii i plaszczyzny
    println!("Punkt przeciecia linii i plaszczyzny: {:?}", l3.calculate_point_of_intersection_with_plane(&p1).unwrap());

    //wyswietl kat pomiedzy linia a plaszczyzna
    println!("Kat pomiedzy linia a plaszczyzna: {:?}", l3.calculate_angle_between_line_and_plane(&p1));

    println!();

    //plaszczyzna 2x-y+z-8=0
    let p2 = plane::Plane::new_p((0.0,0.0,8.0), (1.0,1.0,7.0), (1.0,2.0,8.0));
    //wyswietl plaszczyzne
    println!("Plaszczyzna: {:?}", p2);

    //plaszczyzna 4x + 3y + z + 14 = 0
    let p3 = plane::Plane::new_p((0.0,0.0,-14.0), (1.0,1.0,-21.0), (1.0,2.0,-24.0));
    //wyswietl plaszczyzne
    println!("Plaszczyzna: {:?}", p3);

    //wyswietl czy plaszczyzny sie przecinaja
    println!("Czy plaszczyzny sie przecinaja: {:?}", p2.does_intersect_with_plane(&p3));


    let intersect = p2.calculate_line_of_intersection_with_plane(&p3);
    //wyswietl linie przeciecia plaszczyzn
    println!("Linia przeciecia plaszczyzn: {:?}", intersect.unwrap().display_line());

    //wyswietl kat pomiedzy plaszczyznami
    println!("Kat pomiedzy plaszczyznami: {:?}", p2.calculate_angle_between_planes(&p3));

    println!();

    //linia przechodzaca przez punkt (5,5,4) i (10,10,6)
    let l4 = line3D::Line3D::new_p((5.0, 5.0, 4.0), (10.0, 10.0, 6.0));
    //wyswietl linie
    println!("Linia przechodzaca przez punkty (5,5,4) i (10,10,6): {:?}", l4.display_line());

    //linia przechodzaca przez punkt (5,5,5) i (10,10,3)
    let l5 = line3D::Line3D::new_p((5.0, 5.0, 5.0), (10.0, 10.0, 3.0));
    //wyswietl linie
    println!("Linia przechodzaca przez punkty (5,5,5) i (10,10,3): {:?}", l5.display_line());

    //wyswietl czy linie sie przecinaja
    println!("Czy linie sie przecinaja: {:?}", l4.do_lines_intersect(&l5));

    //wyswietl punk przecięcia linii
    println!("Punkt przecięcia linii: {:?}", l4.calculate_point_of_intersection(&l5).unwrap());

    println!();

    //linia przechodzaca przez punkt (3,-1,-1) i (5,3,-4)
    let l6 = line3D::Line3D::new_p((3.0, -1.0, -1.0), (5.0, 3.0, -4.0));
    //wyswietl linie
    println!("Linia przechodzaca przez punkty (3,-1,-1) i (5,3,-4): {:?}", l6.display_line());

    //sfera o promieniu pierwiastek z 26 i srodku w punkcie (0,0,0)
    let s1 = sphere::Sphere::new((0.0, 0.0, 0.0), f64::sqrt(26.0));
    //wyswietl sfere
    println!("Sfera: {:?}", s1.to_string());

    //wyswietl czy linia i sfera sie przecinaja
    println!("Czy linia i sfera sie przecinaja: {:?}", l6.does_intersect_with_sphere(&s1));

    //wyswietl punkt przeciecia linii i sfery
    println!("Punkt przeciecia linii i sfery: {:?}", l6.calculate_point_of_intersection_with_sphere(&s1));


}
