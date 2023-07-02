use std::rc::Rc;

use serde::{Deserialize, Deserializer};

use crate::{
    tracer::{
        hittable::{HitRecord, Hittable},
        hittable_list::HittableList,
        ray::Ray,
    },
    utils::{
        deserialization_helpers::{deserialize_point, deserialize_vector},
        file_loader,
        mat4::Mat4,
        obj_parser::OBJParser,
        vec3::{Point, Vec3},
    },
};

use super::material::{deserialize_material, Material};

#[derive(Deserialize)]
pub struct Sphere {
    #[serde(deserialize_with = "deserialize_point")]
    position: Point,
    #[serde(rename = "@radius")]
    radius: f32,
    #[serde(rename = "$value")]
    #[serde(deserialize_with = "deserialize_material")]
    pub material: Rc<dyn Material>,
    #[serde(rename = "transform")]
    #[serde(default)]
    pub transformation_matrices: TransformationMatrices,
}

#[derive(Deserialize)]
pub struct Mesh {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "$value")]
    #[serde(deserialize_with = "deserialize_material")]
    pub material: Rc<dyn Material>,
    #[serde(skip_deserializing)]
    pub obj_parser: OBJParser,
    #[serde(rename = "transform")]
    #[serde(default)]
    pub transformation_matrices: TransformationMatrices,
}

pub struct TransformationMatrices {
    pub world_to_object_matrix: Mat4,
    pub normal_matrix: Mat4,
}

impl Default for TransformationMatrices {
    fn default() -> Self {
        Self {
            world_to_object_matrix: Mat4::create(),
            normal_matrix: Mat4::create(),
        }
    }
}

impl Hittable for Mesh {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        const CORRECTION: f32 = 0.00001;
        let transformed_ray = Ray::from_values(
            &self
                .transformation_matrices
                .world_to_object_matrix
                .transform_point3(&ray.origin),
            &self
                .transformation_matrices
                .world_to_object_matrix
                .transform_vec3(&ray.direction),
        );

        for chunk in self.obj_parser.new_index_array.chunks_exact(3) {
            let vertex_a = self.obj_parser.sorted_vertices[chunk[0]];
            let vertex_b = self.obj_parser.sorted_vertices[chunk[1]];
            let vertex_c = self.obj_parser.sorted_vertices[chunk[2]];

            let normal_a = self.obj_parser.sorted_normals[chunk[0]];
            let normal_b = self.obj_parser.sorted_normals[chunk[1]];
            let normal_c = self.obj_parser.sorted_normals[chunk[2]];

            let texture_vertex_1 = self.obj_parser.texture_vertices_to_be_returned[chunk[0]];
            let texture_vertex_2 = self.obj_parser.texture_vertices_to_be_returned[chunk[1]];
            let texture_vertex_3 = self.obj_parser.texture_vertices_to_be_returned[chunk[2]];

            let edge_ab = &vertex_b - &vertex_a;
            let edge_ac = &vertex_c - &vertex_a;

            let p_vec = transformed_ray.direction.cross(&edge_ab);
            let determinant = edge_ac.dot(&p_vec);

            // If dot product of ray direction and triangle normal is 0 then the ray and the triangle are parallel
            // and there's no intersection
            if determinant > -CORRECTION && determinant < CORRECTION {
                continue;
            }

            // Check barycentric coordinates

            let inverse_determinant = 1.0 / determinant;
            let t_vec = &transformed_ray.origin - &vertex_a;
            let u = inverse_determinant * t_vec.dot(&p_vec);

            if u < 0.0 || u > 1.0 {
                continue;
            }

            let q_vec = t_vec.cross(&edge_ac);
            let v = inverse_determinant * transformed_ray.direction.dot(&q_vec);

            if v < 0.0 || u + v > 1.0 {
                continue;
            }

            let t = inverse_determinant * edge_ab.dot(&q_vec);

            if t < t_min || t > t_max {
                continue;
            }

            hit_record.t = t;
            hit_record.point = ray.at(hit_record.t);
            hit_record.material = self.material.clone();

            let outward_normal =
                &(&(&u * &normal_c) + &(&v * &normal_b)) + &(&(1.0 - u - v) * &normal_a);
            let texture_coordinate = &(&(&u * &texture_vertex_3) + &(&v * &texture_vertex_2))
                + &(&(1.0 - u - v) * &texture_vertex_1);

            hit_record.set_face_normal(
                &transformed_ray,
                &self
                    .transformation_matrices
                    .normal_matrix
                    .transform_vec3(&outward_normal),
            );
            hit_record.set_texture_coordinate(&texture_coordinate);

            return true;
        }

        false
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, hit_record: &mut HitRecord) -> bool {
        let transformed_ray = Ray::from_values(
            &self
                .transformation_matrices
                .world_to_object_matrix
                .transform_point3(&ray.origin),
            &self
                .transformation_matrices
                .world_to_object_matrix
                .transform_vec3(&ray.direction),
        );

        let oc = &transformed_ray.origin - &self.position;
        let a = transformed_ray.direction.length_squared();
        let half_b = Vec3::dot(&oc, &transformed_ray.direction);
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

        let transformed_outward_normal = self
            .transformation_matrices
            .normal_matrix
            .transform_vec3(&outward_normal);

        hit_record.set_face_normal(&transformed_ray, &transformed_outward_normal);

        hit_record.material = self.material.clone();

        let u = 0.5
            + f32::atan2(
                transformed_outward_normal.x(),
                transformed_outward_normal.z(),
            ) / (2.0 * std::f32::consts::PI);
        let v = 0.5 - f32::asin(transformed_outward_normal.y()) / std::f32::consts::PI;

        hit_record.set_texture_coordinate(&Vec3::from_values(u, v, 1.0));

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

    let inner_surfaces: InnerSurfaces = InnerSurfaces::deserialize(deserializer)?;

    let surfaces: Vec<Surface> = inner_surfaces.surfaces;
    let mut hittable_list = HittableList::new();

    for surface in surfaces {
        let hittable: Rc<dyn Hittable> = match surface {
            Surface::Sphere(sphere) => Rc::new(sphere) as Rc<dyn Hittable>,
            Surface::Mesh(mut mesh) => {
                mesh.obj_parser.extract_data(
                    &file_loader::load_obj_file(&mesh.name).expect("Reading of OBJ-File failed!"),
                );
                Rc::new(mesh) as Rc<dyn Hittable>
            }
        };
        hittable_list.add(hittable);
    }

    Ok(hittable_list)
}

/*#[derive(Deserialize)]
enum TransformationEnum {
    #[serde(rename = "@translate")]
    Translate {
        #[serde(deserialize_with = "deserialize_vector")]
        translate: Vec3,
    },
    #[serde(rename = "@rotateY")]
    RotateY(f32),
    #[serde(rename = "@scale")]
    Scale {
        #[serde(deserialize_with = "deserialize_vector")]
        scale: Vec3,
    },
}

impl<'de> Deserialize<'de> for TransformationMatrices {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct InnerTransformation {
            #[serde(rename = "$value")]
            transformation: Vec<TransformationEnum>,
        }

        let inner_transformation: InnerTransformation =
            InnerTransformation::deserialize(deserializer)?;


        for t in inner_transformation.transformation {
            match t {
                TransformationEnum::Translate { translate } => {
                    transformation.translate = Some(translate);
                }
                TransformationEnum::RotateY(angle) => {
                    transformation.rotate_y = Some(angle);
                }
                TransformationEnum::Scale { scale } => {
                    transformation.scale = Some(scale);
                }
            }
        }

        Ok(TransformationMatrices::default())
    }
}*/

/*#[derive(Deserialize)]
enum TransformationEnum {
    #[serde(rename = "@translate")]
    Translate(Translate),
    #[serde(rename = "@rotateY")]
    RotateY(RotateY),
    #[serde(rename = "@scale")]
    Scale(Scale),
}

#[derive(Deserialize)]
struct Translate {
    #[serde(deserialize_with = "deserialize_vector")]
    translate: Vec3,
}

#[derive(Deserialize)]
struct RotateY {
    #[serde(rename = "@theta")]
    angle: f32,
}

#[derive(Deserialize)]
struct Scale {
    #[serde(deserialize_with = "deserialize_vector")]
    scale: Vec3,
}

pub trait TransformOperation {}

impl TransformOperation for Translate {}

impl TransformOperation for RotateY {}

impl TransformOperation for Scale {}*/

#[derive(Deserialize)]
pub enum TransformationEnum {
    #[serde(rename = "translate")]
    #[serde(deserialize_with = "deserialize_vector")]
    Translate(Vec3),
    #[serde(rename = "rotateY")]
    RotateY {
        #[serde(rename = "@theta")]
        angle: f32,
    },
    #[serde(rename = "rotateX")]
    RotateX {
        #[serde(rename = "@theta")]
        angle: f32,
    },
    #[serde(rename = "scale")]
    #[serde(deserialize_with = "deserialize_vector")]
    Scale(Vec3),
}

impl<'de> Deserialize<'de> for TransformationMatrices {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize, Default)]
        struct InnerTransformation {
            #[serde(rename = "$value")]
            #[serde(default)]
            transformation: Vec<TransformationEnum>,
        }

        let inner_transformation: InnerTransformation =
            InnerTransformation::deserialize(deserializer)?;

        let transform_operations: Vec<TransformationEnum> = inner_transformation.transformation;

        let world_to_object_transform_matrix =
            Mat4::create_world_to_object_transformation_matrix(&transform_operations);

        Ok(TransformationMatrices {
            world_to_object_matrix: world_to_object_transform_matrix,
            normal_matrix: world_to_object_transform_matrix.transpose(),
        })
    }
}
