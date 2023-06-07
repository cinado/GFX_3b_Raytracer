use std::rc::Rc;

use serde::{Deserialize, Deserializer};

use crate::{
    deserialization_helpers::deserialize_point,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    material::{deserialize_material, Material},
    vec3::{Point, Vec3},
};

#[derive(Deserialize)]
pub struct Sphere {
    #[serde(deserialize_with = "deserialize_point")]
    position: Point,
    #[serde(rename = "@radius")]
    radius: f32,
    #[serde(rename = "$value")]
    #[serde(deserialize_with = "deserialize_material")]
    pub material: Rc<dyn Material>,
}

/*impl Sphere {
    pub fn from_values(point: &Point, radius: &f32) -> Self {
        Self {
            center: *point,
            radius: *radius
        }
    }
}*/

#[derive(Deserialize)]
pub struct Mesh {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$value")]
    #[serde(deserialize_with = "deserialize_material")]
    pub material: Rc<dyn Material>,
}

impl Hittable for Mesh {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut HitRecord,
    ) -> bool {
        // TODO
        false
    }
}

impl Hittable for Sphere {
    fn hit(
        &self,
        ray: &crate::ray::Ray,
        t_min: f32,
        t_max: f32,
        hit_record: &mut HitRecord,
    ) -> bool {
        let oc = &ray.origin - &self.position;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_discriminant = f32::sqrt(discriminant);

        let mut root = (-half_b - sqrt_discriminant) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }
        hit_record.t = root;
        hit_record.point = ray.at(hit_record.t);
        let outward_normal = (&(&hit_record.point - &self.position)) / &self.radius;
        hit_record.set_face_normal(&ray, &outward_normal);

        //Test
        hit_record.material = self.material.clone();

        return true;
    }
}

#[derive(Deserialize)]
enum Surface {
    #[serde(rename = "sphere")]
    Sphere(Sphere),
    #[serde(rename = "mesh")]
    Mesh(Mesh),
}

pub fn deserialize_surfaces<'de, D>(deserializer: D) -> Result<HittableList, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct InnerSurfaces {
        #[serde(rename = "$value")]
        surfaces: Vec<Surface>,
    }

    let inner_surfaces:InnerSurfaces  = InnerSurfaces::deserialize(deserializer)?;

    let surfaces: Vec<Surface> = inner_surfaces.surfaces;
    let mut hittable_list = HittableList::new();

    for surface in surfaces {
        let hittable: Rc<dyn Hittable> = match surface {
            Surface::Sphere(sphere) => Rc::new(sphere) as Rc<dyn Hittable>,
            Surface::Mesh(mesh) => Rc::new(mesh) as Rc<dyn Hittable>,
        };
        hittable_list.add(hittable);
    }

    Ok(hittable_list)
}
