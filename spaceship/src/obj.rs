use tobj;
use nalgebra_glm::{Vec2, Vec3};
use crate::vertex::Vertex;
use crate::color::Color;

pub struct Obj {
    meshes: Vec<Mesh>,
}

struct Mesh {
    vertices: Vec<Vec3>,
    normals: Vec<Vec3>,
    texcoords: Vec<Vec2>,
    indices: Vec<u32>,
    material_color: Color,
}

impl Obj {
    pub fn load(filename: &str) -> Result<Self, tobj::LoadError> {
        let (models, materials) = tobj::load_obj(
            filename,
            &tobj::LoadOptions {
                single_index: true,
                triangulate: true,
                ignore_points: true,
                ignore_lines: true,
                ..Default::default()
            }
        )?;

        let materials = materials?;

        let meshes = models.into_iter().map(|model| {
            let mesh = model.mesh;
            
            // Obtener el color del material si existe
            let material_color = if let Some(mat_id) = mesh.material_id {
                if let Some(material) = materials.get(mat_id) {
                    // Usar el color difuso (Kd) del material
                    if let Some(diffuse) = &material.diffuse {
                        Color::new(
                            (diffuse[0] * 255.0) as u8,
                            (diffuse[1] * 255.0) as u8,
                            (diffuse[2] * 255.0) as u8,
                        )
                    } else {
                        Color::new(128, 128, 128)
                    }
                } else {
                    Color::new(128, 128, 128)
                }
            } else {
                Color::new(128, 128, 128)
            };

            Mesh {
                vertices: mesh.positions.chunks(3)
                    .map(|v| Vec3::new(v[0], -v[1], -v[2]))
                    .collect(),
                normals: mesh.normals.chunks(3)
                    .map(|n| Vec3::new(n[0], -n[1], -n[2]))
                    .collect(),
                texcoords: mesh.texcoords.chunks(2)
                    .map(|t| Vec2::new(t[0], 1.0 - t[1]))
                    .collect(),
                indices: mesh.indices,
                material_color,
            }
        }).collect();

        Ok(Obj { meshes })
    }

    pub fn get_vertex_array(&self) -> Vec<Vertex> {
        let mut vertices = Vec::new();

        for mesh in &self.meshes {
            for &index in &mesh.indices {
                let position = mesh.vertices[index as usize];
                let normal = mesh.normals.get(index as usize)
                    .cloned()
                    .unwrap_or(Vec3::new(0.0, 1.0, 0.0));
                let tex_coords = mesh.texcoords.get(index as usize)
                    .cloned()
                    .unwrap_or(Vec2::new(0.0, 0.0));

                let mut vertex = Vertex::new(position, normal, tex_coords);
                vertex.color = mesh.material_color;
                vertices.push(vertex);
            }
        }

        vertices
    }
}
