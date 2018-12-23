
use gl;
use crate::*;

pub struct ShadedEdges {
    program: program::Program,
    surface: surface::TriangleSurface,
    no_edges: usize,
    pub color: Vec3,
    pub diffuse_intensity: f32,
    pub specular_intensity: f32,
    pub specular_power: f32
}

impl ShadedEdges
{
    pub fn create(gl: &gl::Gl, indices: &[u32], positions: &[f32], tube_radius: f32) -> ShadedEdges
    {
        let program = program::Program::from_resource(&gl, "../Dust/src/objects/shaders/line_shaded",
                                                      "../Dust/src/objects/shaders/shaded").unwrap();

        let x_subdivisions = 1;
        let angle_subdivisions = 10;
        let mut cylinder_positions = Vec::new();
        let mut cylinder_indices = Vec::new();
        for i in 0..x_subdivisions+1 {
            let x = i as f32 / x_subdivisions as f32;
            for j in 0..angle_subdivisions {
                let angle = 2.0 * std::f32::consts::PI * j as f32 / angle_subdivisions as f32;

                cylinder_positions.push(x);
                cylinder_positions.push(angle.cos());
                cylinder_positions.push(angle.sin());
            }
        }
        for i in 0..x_subdivisions as u32 {
            for j in 0..angle_subdivisions as u32 {
                cylinder_indices.push(i * angle_subdivisions as u32 + j);
                cylinder_indices.push(i * angle_subdivisions as u32 + (j+1)%angle_subdivisions as u32);
                cylinder_indices.push((i+1) * angle_subdivisions as u32 + (j+1)%angle_subdivisions as u32);

                cylinder_indices.push(i * angle_subdivisions as u32 + j);
                cylinder_indices.push((i+1) * angle_subdivisions as u32 + (j+1)%angle_subdivisions as u32);
                cylinder_indices.push((i+1) * angle_subdivisions as u32 + j);
            }
        }
        let mut surface = surface::TriangleSurface::create(gl, &cylinder_indices).unwrap();
        surface.add_attributes(&program, &att!["position" => (cylinder_positions, 3)]).unwrap();

        let mut instance_buffer = buffer::VertexBuffer::create(gl).unwrap();

        program.set_used();
        program.setup_attribute("local2worldX", 3, 21, 0, 1).unwrap();
        program.setup_attribute("local2worldY", 3, 21, 3, 1).unwrap();
        program.setup_attribute("local2worldZ", 3, 21, 6, 1).unwrap();
        program.setup_attribute("translation", 3, 21, 9, 1).unwrap();
        program.setup_attribute("normalMatrixX", 3, 21, 12, 1).unwrap();
        program.setup_attribute("normalMatrixY", 3, 21, 15, 1).unwrap();
        program.setup_attribute("normalMatrixZ", 3, 21, 18, 1).unwrap();

        let mut index_pairs = std::collections::HashSet::new();
        for f in 0..indices.len()/3 {
            let i1 = indices[f] as usize;
            let i2 = indices[f+1] as usize;
            let i3 = indices[f+2] as usize;
            index_pairs.insert(if i1 < i2 {(i1, i2)} else {(i2, i1)});
            index_pairs.insert(if i1 < i3 {(i1, i3)} else {(i3, i1)});
            index_pairs.insert(if i2 < i3 {(i2, i3)} else {(i3, i2)});
        }
        let no_edges = index_pairs.len();

        let mut data = Vec::new();
        for (i0, i1) in index_pairs {
            let p0 = vec3(positions[i0 * 3], positions[i0 * 3+1], positions[i0 * 3+2]);
            let p1 = vec3(positions[i1 * 3], positions[i1 * 3+1], positions[i1 * 3+2]);

            let length = (p1 - p0).magnitude();
            let dir = (p1 - p0)/length;
            let local_to_world = rotation_matrix_from_dir_to_dir(vec3(1.0, 0.0, 0.0), dir) * Mat4::from_nonuniform_scale(length, tube_radius, tube_radius);
            let normal_matrix = local_to_world.invert().unwrap().transpose();

            for i in 0..3 {
                for j in 0..3 {
                    data.push(local_to_world[i][j]);
                }
            }

            for i in 0..3 {
                data.push(p0[i]);
            }

            for i in 0..3 {
                for j in 0..3 {
                    data.push(normal_matrix[i][j]);
                }
            }

        }
        instance_buffer.fill_with(&data);

        ShadedEdges { program, surface, no_edges, color: vec3(1.0, 0.0, 0.0), diffuse_intensity: 0.5, specular_intensity: 0.2, specular_power: 5.0 }
    }

    pub fn render(&self, camera: &camera::Camera)
    {
        self.program.cull(state::CullType::BACK);
        self.program.depth_test(state::DepthTestType::LEQUAL);
        self.program.depth_write(true);
        self.program.polygon_mode(state::PolygonType::Fill);

        self.program.add_uniform_float("diffuse_intensity", &self.diffuse_intensity).unwrap();
        self.program.add_uniform_float("specular_intensity", &self.specular_intensity).unwrap();
        self.program.add_uniform_float("specular_power", &self.specular_power).unwrap();

        self.program.add_uniform_int("use_texture", &0).unwrap();
        self.program.add_uniform_vec3("color", &self.color).unwrap();

        self.program.add_uniform_mat4("viewMatrix", camera.get_view()).unwrap();
        self.program.add_uniform_mat4("projectionMatrix", camera.get_projection()).unwrap();
        self.surface.render_instances(self.no_edges).unwrap();
    }
}
