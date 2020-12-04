//          _____              ________                         
//         / __  \____  __  __/__  ___/_______  ________  _____
//        / /_/  / __ `/ / / /  / / / ___/ __ `/ ___/ _ \/ ___/
//       /  _,  / /_/ / /_/ /  / / / /  / /_/ / /__/  __/ /    
//      /_/  |_|\__,_/\__, /  /_/ /_/   \__,_/\___/\___/_/     
//                    /___/                                  


mod geometri;
mod vector3;
mod intersnect;
mod scene;
mod ray;

use std::time::Instant;

use crate::geometri::*;
use crate::vector3::*;
use crate::scene::*;



// implementera trait fr grejjer i scenen. tack eskil!<3 undviker code dupes


/*
#[test]
fn test_can_reader_scene() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0
            },
            radius: 1.0,
            color: Color {
                red: 0.4,
                green: 1.0,
                blue: 0.4,
                alpha: 1.0,
            },
        },
    };

    let img: DynamicImage = render(&scene);
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());

}
*/





fn main() {
    let start_tid = Instant::now();


    println!("Hello, world!");

    let mut min_scen = Scen {
        width: 80,
        height: 60,
        fov: 85.0,
        elements: vec![],
        ljus: Ljus {position: Point{x: 0.0, y: 20.0, z: -20.0}, color: Color {red: 255, green: 255, blue: 255, alpha: 255}, intensitet: 1.0},
    };

    //röd
    min_scen.lagg_till_rekvisita(Rekvisita::Sphere(Sphere{
        center: Point{x: 0.0, y: 0.0, z: -5.0},
        radius: 1.5,
        material: Material{albedo: 1.0,
            color: Color{red: 233, green: 75, blue: 60, alpha: 255} }}
            ));

    //blå
    min_scen.lagg_till_rekvisita(Rekvisita::Sphere(Sphere{
        center: Point{x: 4.3, y: 7.0, z: -9.0},
        radius: 4.0,
        material: Material{albedo: 1.0,
            color: Color{red: 111, green: 159, blue: 216, alpha: 255} }}
            ));

    //grön
    min_scen.lagg_till_rekvisita(Rekvisita::Sphere(Sphere{
        center: Point{x: -8.0, y: 5.0, z: -11.0},
        radius: 5.0,
        material: Material{albedo: 1.0,
            color: Color{red: 210, green: 105, blue: 30, alpha: 255} }}
            ));

    min_scen.lagg_till_rekvisita(Rekvisita::Plane(Plane{
        position: Point{x: 3.0, y: -3.0, z: 0.0},
        normal: Vector3{x: 0.0, y: -1.0, z: 0.0},
        material: Material{albedo: 1.0,
            color: Color{red: 120, green: 150, blue: 88, alpha: 255} }}
            ));

    render(&min_scen).save("helloworld.png").unwrap();

    let tid = start_tid.elapsed().as_micros() as f64 / 1000000.0;
    println!("tid: {}", tid);

}
