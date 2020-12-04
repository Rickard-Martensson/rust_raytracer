use crate::geometri::*;
use crate::intersnect::*;
use crate::ray::*;
use image::*;

pub struct Scen {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    //pub sphere: Sphere,
    //pub spheres: Vec<Sphere>,
    pub elements: Vec<Rekvisita>,
    pub ljus: Ljus,

}

impl Scen {
    pub fn trace(&self, ray: &Ray) -> Option<Korsning> {
        return self.elements
            .iter()
            .filter_map(|elem| elem.option_korsning(ray)
            .map(|avstand| Korsning::new(avstand, elem))
            .filter(|elem| elem.avstand > 1e-6))
            .min_by(|i1, i2| i1.avstand.partial_cmp(&i2.avstand)
            .unwrap());
    }

    
    

    pub fn lagg_till_rekvisita(&mut self, object: Rekvisita) {
        self.elements.push(object);
    }
}



fn jobbig_fargkonvert(color: &Color, ljusfarg: &Color, styrka: f64, reflekt: f64) -> image::Rgba<u8> {
    return Rgba::from_channels(

        ((color.red as f64 * ljusfarg.red as f64).sqrt() * styrka * reflekt) as u8,
        ((color.green as f64 * ljusfarg.green as f64).sqrt() * styrka * reflekt) as u8,
        ((color.blue as f64 * ljusfarg.blue as f64).sqrt() * styrka * reflekt) as u8,
        ((color.alpha as f64 * ljusfarg.alpha as f64).sqrt() * styrka * reflekt) as u8,
        );
}

pub fn render(scene: &Scen) -> DynamicImage {
    let mut image = DynamicImage::new_rgb8(scene.width, scene.height);
    let black = Rgba::from_channels(0,0,0,255);
    let _white = Rgba::from_channels(195,195,195,0);
    let background = Rgba::from_channels(48,48,48,255);
    for x in 0..scene.width {
        for y in 0..scene.height {
            let lokal_ray = Ray::create_prime(x, y, scene);

            let lokalt_objekt = scene.trace(&lokal_ray);
            match lokalt_objekt {
                Some(lokalt_objekt) => {
                    let lokal_traffpunkt = lokal_ray.origin + (lokal_ray.direction * lokalt_objekt.avstand);
                    let ljus_riktning = lokal_traffpunkt - scene.ljus.position;

                    let skugg_ray = Ray {
                        origin: lokal_traffpunkt,
                        direction: -ljus_riktning.normalisera(),
                    };
                    let traffpunkt_skuggad = !scene.trace(&skugg_ray).is_some();
                    let ljus_albedo = lokalt_objekt.object.albedo();

                    if traffpunkt_skuggad {
                        let lokal_yta_normal = lokalt_objekt.object.yta_normal(&lokal_traffpunkt);

                        let ljus_styrka_skalar = -lokal_yta_normal.normalisera().skalarprodukt(&ljus_riktning.normalisera()).min(0.0);
                        //println!("skalarprodukt: {}", ljus_styrka_skalar);
                        


                        let lokal_farg = jobbig_fargkonvert(lokalt_objekt.object.color(), &scene.ljus.color, ljus_styrka_skalar, ljus_albedo);
                        image.put_pixel(x, y, lokal_farg);
                    } else {
                        //let lokal_skuggfarg = jobbig_fargkonvert(lokalt_objekt.object.color(), &scene.ljus.color, 0.05, ljus_albedo);
                        image.put_pixel(x, y, black);
                    }
                },
                None => image.put_pixel(x, y, background),
            }
            
        }
    }
    
    /*
    for x in 0..scene.width {
        for y in 0..scene.height {
            let lokal_pixel = image.get_pixel_mut(x, y);
            let (a, b, c, d) = lokal_pixel.from_channels();


        }
    }
    */

    return image;
}