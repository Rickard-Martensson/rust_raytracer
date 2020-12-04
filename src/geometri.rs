use crate::vector3::*;


pub enum Rekvisita {
    Sphere(Sphere),
    Plane(Plane),
}

impl Rekvisita {
    pub fn color(&self) -> &Color {
        match *self {
            Rekvisita::Sphere(ref sfar) => &sfar.material.color,
            Rekvisita::Plane(ref plan) => &plan.material.color,
        }
    }
    pub fn albedo(&self) -> f64 {
        match *self {
            Rekvisita::Sphere(ref sfar) => sfar.material.albedo,
            Rekvisita::Plane(ref plan) => plan.material.albedo,
        }
    }
    /*
    pub fn elem_korsning(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Rekvisita::Sphere(ref sfar) => sfar.option_korsning(ray),
            Rekvisita::Plane(ref plan) => plan.option_korsning(ray),
        }
    }

    */
}

pub struct Material {
    pub color: Color,
    pub albedo: f64,
}

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

#[derive(Copy, Clone, Default)]
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
            z: 5.0,
        }
    }
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Material,

}

impl Sphere {
    pub fn ny(center: Point, radius: f64, material: Material) -> Self {
        return Self { center, radius, material};
    }
}

pub struct Plane {
    pub position: Point,
    pub normal: Vector3,
    pub material: Material,
}

impl Plane {
    pub fn ny(position: Point, normal: Vector3, material: Material) -> Self {
        return Self {position, normal, material};
    }
}

pub struct Ljus {
    pub position: Point,
    pub color: Color,
    pub intensitet: f64,
}