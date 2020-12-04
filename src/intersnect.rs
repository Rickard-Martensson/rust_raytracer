use crate::vector3::*;
use crate::geometri::*;
use crate::ray::*;

#[derive(Copy, Clone)]
pub struct Korsning<'a> {   //implementera lifetime senare, för o optimera
    pub avstand: f64,
    pub object: &'a Rekvisita,
    pub luddighet: f64,

}

impl<'a> Korsning<'a> {
    pub fn new<'b>(avstand: f64, object: &'b Rekvisita) -> Korsning<'b> {
        return Korsning {
            avstand: avstand,
            object: object,
            luddighet: 0.0,
        }
    }
}

pub trait Korsningsbar {
    fn option_korsning(&self, ray: &Ray) -> Option<f64>;
    fn yta_normal(&self, traffpunkt: &Point) -> Vector3; 
}

impl Korsningsbar for Rekvisita {
    fn option_korsning(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Rekvisita::Sphere(ref sfar) => sfar.option_korsning(ray),
            Rekvisita::Plane(ref plan) => plan.option_korsning(ray),
        }
    }
    fn yta_normal(&self, traffpunkt: &Point) -> Vector3 {
        match *self {
            Rekvisita::Sphere(ref sfar) => sfar.yta_normal(traffpunkt),
            Rekvisita::Plane(ref plan) => plan.yta_normal(traffpunkt),
        }

    }
}

impl Korsningsbar for Sphere {
    fn option_korsning(&self, ray: &Ray) -> Option<f64> {
        // return false;
        let katet_a: Vector3 = self.center - ray.origin;
        let katet_b = katet_a.skalarprodukt(&ray.direction);
        let hypotenusa_c = katet_a.skalarprodukt(&katet_a) - (katet_b * katet_b);
        let radie = self.radius * self.radius;
        if hypotenusa_c > radie {
            return None;
        }

        let korda = (radie - hypotenusa_c).sqrt();
        let cirkel_punkt_a = katet_b - korda;
        let cirkel_punkt_b = katet_b + korda;

        if cirkel_punkt_a < 0.0 && cirkel_punkt_b < 0.0 {
            return None;
        }

        let avstand = if cirkel_punkt_a < cirkel_punkt_b {cirkel_punkt_b} else {cirkel_punkt_a};
        return Some(avstand);
        

    }
    fn yta_normal(&self, traffpunkt: &Point) -> Vector3 {
        return *traffpunkt - self.center;
    }
}

impl Korsningsbar for Plane {
    fn option_korsning(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denom = normal.skalarprodukt(&ray.direction);
        if denom > 1e-6 {
            let ny_vektor = self.position - ray.origin;
            let avstand = ny_vektor.skalarprodukt(&normal) / denom;
            if avstand >= 0.0 {
                return Some(avstand);
            }
        }
        return None;
    }
    fn yta_normal(&self, _: &Point) -> Vector3 {
        return -self.normal;
    }

}