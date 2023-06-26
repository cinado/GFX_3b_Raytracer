use super::vec3::Vec3;

pub struct OBJParser {
    vec_normals: Vec<Vec3>,
    vertices: Vec<Vec3>,
    texture_vertices: Vec<Vec3>,
    pub texture_vertices_to_be_returned: Vec<Vec3>,
    current_index: usize,
    pub sorted_vertices: Vec<Vec3>,
    pub sorted_normals: Vec<Vec3>,
    pub new_index_array: Vec<usize>,
    already_mapped_triples: std::collections::HashMap<Vec<String>, usize>,
}

impl OBJParser {
    pub fn new() -> OBJParser {
        OBJParser {
            vec_normals: Vec::new(),
            vertices: Vec::new(),
            texture_vertices: Vec::new(),
            texture_vertices_to_be_returned: Vec::new(),
            current_index: 0,
            sorted_vertices: Vec::new(),
            sorted_normals: Vec::new(),
            new_index_array: Vec::new(),
            already_mapped_triples: std::collections::HashMap::new(),
        }
    }

    pub fn extract_data(&mut self, data: &str) -> (Vec<Vec3>, Vec<Vec3>, Vec<usize>, Vec<Vec3>) {
        let lines = data.split("\n");
        for line in lines {
            let elements: Vec<&str> = line.trim().split_whitespace().collect();

            if elements.is_empty() {
                continue;
            }
            
            match elements[0] {
                "vn" => self.add_normal(elements[1].parse().unwrap(), elements[2].parse().unwrap(), elements[3].parse().unwrap()),
                "v" => self.add_vertex(elements[1].parse().unwrap(), elements[2].parse().unwrap(), elements[3].parse().unwrap()),
                "vt" => self.texture_vertices.push(Vec3::from_values(elements[1].parse().unwrap(), elements[2].parse().unwrap(), 1.0)),
                "f" => self.process_indices(&elements[1..]),
                _ => (),
            }
        }

        (
            self.sorted_vertices.clone(),
            self.sorted_normals.clone(),
            self.new_index_array.clone(),
            self.texture_vertices_to_be_returned.clone(),
        )
    }

    pub fn add_vertex(&mut self, x_component: f32, y_component: f32, z_component: f32) {
        self.vertices.push(Vec3::from_values(x_component,y_component, z_component));
    }

    pub fn add_normal(&mut self, x_component: f32, y_component: f32, z_component: f32) {
        self.vec_normals.push(Vec3::from_values(x_component, y_component, z_component));
    }

    pub fn process_indices(&mut self, elements: &[&str]) {
        for element in elements {
            let index: Vec<String> = element.split('/').map(|s| s.to_string()).collect();
            if self.already_mapped_triples.contains_key(&index) {
                self.new_index_array.push(*self.already_mapped_triples.get(&index).unwrap());
            } else {
                let vertex_index = index[0].parse::<usize>().unwrap() - 1;
                let normal_index = index[2].parse::<usize>().unwrap() - 1;
                self.sorted_vertices.push(self.vertices[vertex_index]);
                self.sorted_normals.push(self.vec_normals[normal_index]);

                if let Some(texture_index) = index.get(1) {
                    let texture_vertex = self.texture_vertices[texture_index.parse::<usize>().unwrap() - 1];
                    self.texture_vertices_to_be_returned.push(texture_vertex);
                }

                self.new_index_array.push(self.current_index);
                self.already_mapped_triples.insert(index, self.current_index);
                self.current_index += 1;
            }
        }
    }
}

impl Default for OBJParser {
    fn default() -> Self {
        OBJParser::new()
    }
}