use crate::geometri::*;

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn zero() -> Self {
        return Vector3{
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

    }

    pub fn normal_vektor(&self) -> Vector3 {
        return Vector3 {
            x: 0.0,
            y: 1.0,
            z: 0.0
        }
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


impl std::ops::Sub for Point {
    type Output = Vector3;

    fn sub(self, point2: Self) -> Vector3 {
        return Vector3 {
            x: self.x - point2.x,
            y: self.y - point2.y,
            z: self.z - point2.z,
        };
    }
    
}

impl std::ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Vector3 {
        return Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        };
    }
}

impl std::ops::Mul<f64> for Vector3{
    type Output = Vector3;

    fn mul(self, skalar: f64) -> Vector3 {
        return Vector3 {
            x: self.x * skalar,
            y: self.y * skalar,
            z: self.z * skalar,
        };
    }
}

impl std::ops::Add<Vector3> for Point{
    type Output = Point;

    fn add(self, vektor: Vector3) -> Point {
        return Point {
            x: self.x + vektor.x,
            y: self.y + vektor.y,
            z: self.z + vektor.z,
        }
    }
}