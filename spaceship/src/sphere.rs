use nalgebra_glm::{Vec2, Vec3};
use std::f32::consts::PI;
use crate::vertex::Vertex;

pub fn create_sphere(radius: f32, rings: u32, sectors: u32) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    
    let r = 1.0 / (rings - 1) as f32;
    let s = 1.0 / (sectors - 1) as f32;

    for ring in 0..rings {
        for sector in 0..sectors {
            let theta = PI * ring as f32 * r;
            let phi = 2.0 * PI * sector as f32 * s;
            
            let x = theta.sin() * phi.cos();
            let y = theta.cos();
            let z = theta.sin() * phi.sin();

            let position = Vec3::new(x * radius, y * radius, z * radius);
            let normal = Vec3::new(x, y, z).normalize();
            let tex_coords = Vec2::new(sector as f32 * s, ring as f32 * r);

            vertices.push(Vertex::new(position, normal, tex_coords));
        }
    }

    let mut triangulated = Vec::new();
    for ring in 0..rings - 1 {
        for sector in 0..sectors - 1 {
            let current_row = ring * sectors;
            let next_row = (ring + 1) * sectors;

            let v1 = vertices[(current_row + sector) as usize].clone();
            let v2 = vertices[(next_row + sector) as usize].clone();
            let v3 = vertices[(next_row + sector + 1) as usize].clone();
            
            triangulated.push(v1);
            triangulated.push(v2);
            triangulated.push(v3);

            let v4 = vertices[(current_row + sector) as usize].clone();
            let v5 = vertices[(next_row + sector + 1) as usize].clone();
            let v6 = vertices[(current_row + sector + 1) as usize].clone();
            
            triangulated.push(v4);
            triangulated.push(v5);
            triangulated.push(v6);
        }
    }

    triangulated
}
