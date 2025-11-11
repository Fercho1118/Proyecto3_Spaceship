use crate::vertex::Vertex;
use nalgebra_glm::{Vec2, Vec3};

pub fn create_ring(inner_radius: f32, outer_radius: f32, segments: u32) -> Vec<Vertex> {
    let mut vertices = Vec::new();
    
    for i in 0..segments {
        let theta1 = (i as f32 / segments as f32) * std::f32::consts::PI * 2.0;
        let theta2 = ((i + 1) as f32 / segments as f32) * std::f32::consts::PI * 2.0;
        
        let cos1 = theta1.cos();
        let sin1 = theta1.sin();
        let cos2 = theta2.cos();
        let sin2 = theta2.sin();
        
        let inner1 = Vec3::new(inner_radius * cos1, 0.0, inner_radius * sin1);
        let inner2 = Vec3::new(inner_radius * cos2, 0.0, inner_radius * sin2);
        
        let outer1 = Vec3::new(outer_radius * cos1, 0.0, outer_radius * sin1);
        let outer2 = Vec3::new(outer_radius * cos2, 0.0, outer_radius * sin2);
        
        let normal = Vec3::new(0.0, 1.0, 0.0);
        let tex = Vec2::new(0.0, 0.0);
        
        vertices.push(Vertex::new(inner1, normal, tex));
        vertices.push(Vertex::new(outer1, normal, tex));
        vertices.push(Vertex::new(inner2, normal, tex));
        
        vertices.push(Vertex::new(inner2, normal, tex));
        vertices.push(Vertex::new(outer1, normal, tex));
        vertices.push(Vertex::new(outer2, normal, tex));
    }
    
    vertices
}
