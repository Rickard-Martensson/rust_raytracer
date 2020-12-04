//use image::DynamicImage;
use image::*;
//use std::num::sqrt;
use std::ops::{Add, Sub};
use image::Pixel;
use image::RgbaImage;

pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}



#[derive(Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn zero() -> Self {
        Point{
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl std::ops::Sub for Point {
    type Output = Vector3;

    fn sub(self, point2: Self) -> Vector3 {
        Vector3 {
            x: self.x - point2.x,
            y: self.y - point2.y,
            z: self.z - point2.z,
        }
    }
}

impl Vector3 {
    pub fn zero() -> Self {
        return Vector3{
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

    }

    pub fn magnitud(&self) -> f64 {
        return (
        self.x * self.x
        + self.y * self.y
        + self.z * self.z
        ).sqrt();

    }

    pub fn normalisera(&self) -> Vector3 {
        return Vector3{
            x: self.x / self.magnitud(),
            y: self.y / self.magnitud(),
            z: self.z / self.magnitud(),
        }
    }

    //pub fn vinkelskillnad(&self, ny_vektor: Vector3) {

    //}

    pub fn skalarprodukt(&self, ny_vektor: &Vector3) -> f64 {
        return self.x * ny_vektor.x 
        + self.y * ny_vektor.y 
        + self.z * ny_vektor.z
    } 


}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,

}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub sphere: Sphere,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        // return false;
        let katet_a: Vector3 = self.center - ray.origin;
        let katet_b = katet_a.skalarprodukt(&ray.direction);
        let hypotenusa_c = katet_a.skalarprodukt(&katet_a) - (katet_b * katet_b);

        return hypotenusa_c < (self.radius * self.radius)
    }
}

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    
    pub fn create_prime(x: u32, y: u32, scene: &Scene) -> Ray {
        assert!(scene.width > scene.height);

        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();

        let sensor_x = (((
            (x as f64 + 0.5)
            / scene.width as f64)
            * 2.0 - 1.0)
            * aspect_ratio)
            * fov_adjustment;

        let sensor_y = (1.0 - ((
            (y as f64 + 0.5)
            / scene.height as f64)
            * 2.0))
            * fov_adjustment;

        Ray {
           origin: Point::zero(),
           direction: Vector3 {
               x: sensor_x,
               y: sensor_y,
               z: -1.0,
           }           
           .normalisera(),

        }
    }
}


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

pub fn to_rgba(color: Color) -> u8 {
    return (255.0 * ((255.0 * (255.0 * color.red + color.green)) + color.blue)) as u8
}

pub fn render(scene: &Scene) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    //let black = Rgba::from_channels(0,0,0,0);
    //let white = Rgba::from_channels(195,195,195,0);
    let background = Rgba::from_channels(32,32,32,32);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let ray = Ray::create_prime(x, y, scene);

            if scene.sphere.intersect(&ray) {
                let local_color = Rgba::from_channels(
                (255.0 * &scene.sphere.color.red) as u8,
                (255.0 * &scene.sphere.color.green) as u8,
                (255.0 * &scene.sphere.color.blue) as u8,
                (255.0 * &scene.sphere.color.alpha) as u8,
                );
                image.put_pixel(x, y, local_color)
                
                //image.put_pixel(x, y, to_rgba(&scene.sphere.color))
            } else {
                image.put_pixel(x, y, background)
            }
        }
    }
    image
}

fn main() {
    println!("Hello, world!");

    let min_scen = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        sphere: Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0
            },
            radius: 2.0,
            color: Color {
                red: 0.012,
                green: 0.286,
                blue: 0.553,
                alpha: 1.0,
            },
        },
    };

    render(&min_scen).save("helloworld.png").unwrap();
    
}
