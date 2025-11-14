use nalgebra_glm::{Vec2, Vec3};
use crate::fragment::Fragment;
use crate::vertex::Vertex;
use crate::color::Color;
use crate::Uniforms;

pub enum ShaderType {
    Sun,
    RockyPlanet,
    GasGiant,
    EarthLike,
    IcePlanet,
    Moon,
    Rings,
    Spaceship,  // Nuevo shader para la nave
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms, shader_type: &ShaderType, vertex_position: &Vec3, vertex_normal: &Vec3) -> Color {
    match shader_type {
        ShaderType::Sun => sun_shader(fragment, uniforms, vertex_position, vertex_normal),
        ShaderType::RockyPlanet => rocky_planet_shader(fragment, uniforms, vertex_position, vertex_normal),
        ShaderType::GasGiant => gas_giant_shader(fragment, uniforms, vertex_position, vertex_normal),
        ShaderType::EarthLike => earth_like_shader(fragment, uniforms, vertex_position, vertex_normal),
        ShaderType::IcePlanet => ice_planet_shader(fragment, uniforms, vertex_position, vertex_normal),
        ShaderType::Moon => moon_shader(fragment, uniforms, vertex_position, vertex_normal),
        ShaderType::Rings => rings_shader(fragment, uniforms, vertex_position, vertex_normal),
        ShaderType::Spaceship => spaceship_shader(fragment, vertex_normal),
    }
}

fn sun_shader(_fragment: &Fragment, uniforms: &Uniforms, _vertex_position: &Vec3, vertex_normal: &Vec3) -> Color {
    let time = uniforms.time as f32 * 0.5;
    
    let zoom = 100.0;
    let x = vertex_normal.x;
    let y = vertex_normal.y;
    
    // 3 capas de ruido para superficie solar dinámica
    let noise_value1 = (x * zoom + time).sin() * (y * zoom + time).cos();
    let noise_value2 = ((x + 0.5) * zoom * 1.5 - time * 0.8).sin() * ((y + 0.5) * zoom * 1.5 + time * 0.8).cos();
    let noise_value3 = ((x * 0.7 + y * 0.3) * zoom * 0.8 + time * 1.2).sin();
    
    let combined_noise = (noise_value1 + noise_value2 + noise_value3) / 3.0;
    let brightness = (combined_noise + 1.0) * 0.5;
    
    let core_white = Color::new(255, 255, 240);
    let bright_yellow = Color::new(255, 220, 100);
    let yellow = Color::new(255, 180, 50);
    let orange = Color::new(255, 100, 0);
    let deep_red = Color::new(200, 30, 0);
    
    if brightness > 0.85 {
        core_white
    } else if brightness > 0.7 {
        bright_yellow
    } else if brightness > 0.5 {
        yellow
    } else if brightness > 0.3 {
        orange
    } else {
        deep_red
    }
}

fn rocky_planet_shader(_fragment: &Fragment, uniforms: &Uniforms, vertex_position: &Vec3, vertex_normal: &Vec3) -> Color {
    let time = uniforms.time as f32 * 0.05;
    
    let zoom = 40.0;
    let x = vertex_position.x;
    let y = vertex_position.y;
    let z = vertex_position.z;
    
    // 4 capas de ruido para textura rocosa
    let noise1 = (x * zoom + time * 0.1).sin() * (y * zoom).cos() * (z * zoom).sin();
    let noise2 = ((x + 0.3) * zoom * 0.7).cos() * ((y + 0.7) * zoom * 0.9).sin();
    let noise3 = ((z + 0.5) * zoom * 1.3).sin() * ((x - 0.2) * zoom * 0.5).cos();
    let noise4 = ((x * y * zoom * 0.3).sin() + (z * y * zoom * 0.4).cos()) * 0.5;
    
    let combined = noise1 * 0.4 + noise2 * 0.3 + noise3 * 0.2 + noise4 * 0.1;
    
    let dark_rock = Color::new(40, 30, 25);
    let medium_rock = Color::new(80, 60, 45);
    let light_rock = Color::new(120, 90, 65);
    let sandy = Color::new(160, 130, 90);
    let light_sand = Color::new(190, 160, 120);
    
    let crater_pattern = ((x * 25.0).sin() * (z * 25.0).cos() + 1.0) * 0.5;
    
    let base_color = if combined > 0.5 {
        light_sand
    } else if combined > 0.2 {
        sandy
    } else if combined > -0.1 {
        light_rock
    } else if combined > -0.4 {
        medium_rock
    } else {
        dark_rock
    };
    
    let light_dir = Vec3::new(0.5, 0.8, -0.5).normalize();
    let intensity = vertex_normal.dot(&light_dir).max(0.0) * 0.7 + 0.3;
    
    let crater_shadow = if crater_pattern > 0.85 { 0.7 } else { 1.0 };
    
    Color::new(
        (base_color.to_hex() >> 16 & 0xFF) as u8,
        (base_color.to_hex() >> 8 & 0xFF) as u8,
        (base_color.to_hex() & 0xFF) as u8,
    ) * intensity * crater_shadow
}

fn gas_giant_shader(_fragment: &Fragment, uniforms: &Uniforms, vertex_position: &Vec3, vertex_normal: &Vec3) -> Color {
    let time = uniforms.time as f32 * 0.2;
    
    let y = vertex_position.y;
    let x = vertex_position.x;
    let z = vertex_position.z;
    
    // Bandas atmosféricas
    let bands = ((y * 10.0 + time * 0.5).sin() + 1.0) * 0.5;
    let bands2 = ((y * 15.0 - time * 0.3).sin() + 1.0) * 0.5;
    
    let swirl = ((x * 15.0 + z * 8.0 + y * 20.0 - time * 3.0).sin() + 1.0) * 0.5;
    let turbulence = ((x * 25.0).cos() * (z * 20.0).sin() * (y * 10.0).cos()) * 0.3;
    
    // Tormenta circular 
    let storm_x = 0.3;
    let storm_y = 0.0;
    let storm_radius = 0.4;
    let dist_to_storm = ((x - storm_x).powi(2) + (y - storm_y).powi(2)).sqrt();
    let storm = if dist_to_storm < storm_radius {
        ((storm_radius - dist_to_storm) / storm_radius) * 0.6
    } else {
        0.0
    };
    
    let deep_blue = Color::new(15, 40, 100);
    let blue = Color::new(45, 80, 160);
    let light_blue = Color::new(80, 120, 200);
    let pale_blue = Color::new(120, 160, 230);
    let white = Color::new(200, 220, 255);
    
    let combined_bands = (bands * 0.6 + bands2 * 0.4 + turbulence).clamp(0.0, 1.0);
    
    let base_color = if storm > 0.3 {
        white
    } else if combined_bands > 0.75 {
        white
    } else if combined_bands > 0.6 {
        pale_blue
    } else if combined_bands > 0.4 {
        light_blue
    } else if combined_bands > 0.25 {
        blue
    } else {
        deep_blue
    };
    
    let swirl_factor = swirl * 0.15;
    
    let light_dir = Vec3::new(0.3, 0.7, -0.6).normalize();
    let intensity = vertex_normal.dot(&light_dir).max(0.0) * 0.6 + 0.4;
    
    Color::new(
        (base_color.to_hex() >> 16 & 0xFF) as u8,
        (base_color.to_hex() >> 8 & 0xFF) as u8,
        (base_color.to_hex() & 0xFF) as u8,
    ) * (intensity + storm + swirl_factor)
}

fn earth_like_shader(_fragment: &Fragment, uniforms: &Uniforms, vertex_position: &Vec3, vertex_normal: &Vec3) -> Color {
    let time = uniforms.time as f32 * 0.03;
    
    let zoom = 30.0;
    let x = vertex_position.x;
    let y = vertex_position.y;
    let z = vertex_position.z;
    
    // 4 capas: continentes, océanos, vegetación, nubes
    let continent_noise = (x * zoom * 0.5 + time * 0.1).sin() * (z * zoom * 0.5).cos() * (y * zoom * 0.3).sin();
    let ocean_depth = ((x + 0.5) * zoom * 0.8).cos() * ((z - 0.3) * zoom * 0.7).sin();
    let vegetation = ((x * zoom * 1.5).sin() + (z * zoom * 1.3).cos() + (y * zoom * 1.1).sin()) * 0.3;
    let clouds = ((x * zoom * 2.0 + time * 2.0).sin() * (y * zoom * 2.5 - time * 1.5).cos() * (z * zoom * 2.2 + time).sin());
    let cloud_factor = (clouds + 1.0) * 0.5;
    
    let combined = continent_noise + ocean_depth * 0.3 + vegetation * 0.2;
    
    let deep_ocean = Color::new(10, 40, 100);
    let ocean = Color::new(20, 80, 150);
    let shallow_water = Color::new(40, 120, 180);
    let beach = Color::new(200, 180, 120);
    let land = Color::new(60, 120, 40);
    let mountain = Color::new(100, 90, 70);
    let snow = Color::new(240, 240, 255);
    let cloud_white = Color::new(255, 255, 255);
    
    let base_color = if combined > 0.6 {
        snow
    } else if combined > 0.4 {
        mountain
    } else if combined > 0.15 {
        land
    } else if combined > 0.0 {
        beach
    } else if combined > -0.3 {
        shallow_water
    } else if combined > -0.6 {
        ocean
    } else {
        deep_ocean
    };
    
    let light_dir = Vec3::new(0.5, 0.7, -0.5).normalize();
    let intensity = vertex_normal.dot(&light_dir).max(0.0) * 0.7 + 0.3;
    
    // Mezcla de nubes con superficie
    let final_color = if cloud_factor > 0.7 {
        let blend = (cloud_factor - 0.7) / 0.3;
        Color::new(
            ((base_color.to_hex() >> 16 & 0xFF) as f32 * (1.0 - blend) + (cloud_white.to_hex() >> 16 & 0xFF) as f32 * blend) as u8,
            ((base_color.to_hex() >> 8 & 0xFF) as f32 * (1.0 - blend) + (cloud_white.to_hex() >> 8 & 0xFF) as f32 * blend) as u8,
            ((base_color.to_hex() & 0xFF) as f32 * (1.0 - blend) + (cloud_white.to_hex() & 0xFF) as f32 * blend) as u8,
        )
    } else {
        base_color
    };
    
    Color::new(
        (final_color.to_hex() >> 16 & 0xFF) as u8,
        (final_color.to_hex() >> 8 & 0xFF) as u8,
        (final_color.to_hex() & 0xFF) as u8,
    ) * intensity
}

fn ice_planet_shader(_fragment: &Fragment, uniforms: &Uniforms, vertex_position: &Vec3, vertex_normal: &Vec3) -> Color {
    let time = uniforms.time as f32 * 0.15;
    
    let zoom = 45.0;
    let x = vertex_position.x;
    let y = vertex_position.y;
    let z = vertex_position.z;
    
    // 5 capas: cristales de hielo, grietas, escarcha, auroras polares, niebla
    let ice_crystals = (x * zoom * 1.5 + time * 0.5).sin() * (z * zoom * 1.3 - time * 0.3).cos();
    let cracks = ((x * zoom * 4.0).sin() * (z * zoom * 3.5).cos() + (y * zoom * 3.8).sin()).abs();
    let frozen_waves = ((y * zoom * 2.0).sin() + (x * zoom * 1.8).cos()) * 0.5;
    let frost = (x * zoom * 3.0).cos() * (z * zoom * 2.5).sin() * (y * zoom * 2.8).cos();
    
    // Efecto de auroras en los polos
    let polar_dist = y.abs();
    let aurora = if polar_dist > 0.6 {
        ((x * zoom * 5.0 + time * 2.0).sin() * (z * zoom * 5.0 - time * 2.0).cos() + 1.0) * 0.5 * (polar_dist - 0.6) * 3.0
    } else {
        0.0
    };
    
    let haze = ((x + y + z) * zoom * 0.8 + time).sin() * 0.3;
    
    let combined = ice_crystals * 0.3 + frozen_waves * 0.25 + frost * 0.2 + haze * 0.15 + cracks * 0.1;
    let brightness = (combined + 1.0) * 0.5;
    
    // Colores de hielo
    let deep_ice = Color::new(60, 100, 150);
    let ice = Color::new(100, 150, 200);
    let cyan_ice = Color::new(130, 200, 230);
    let pale_ice = Color::new(170, 220, 245);
    let bright_ice = Color::new(200, 235, 255);
    let pure_white = Color::new(245, 250, 255);
    
    // Colores de aurora (cyan-green)
    let aurora_cyan = Color::new(100, 255, 220);
    
    let mut base_color = if cracks < 0.3 {
        deep_ice
    } else if brightness > 0.75 {
        pure_white
    } else if brightness > 0.6 {
        bright_ice
    } else if brightness > 0.45 {
        pale_ice
    } else if brightness > 0.3 {
        cyan_ice
    } else if brightness > 0.2 {
        ice
    } else {
        deep_ice
    };
    
    // Mezcla de efecto aurora
    if aurora > 0.1 {
        let aurora_blend = aurora.min(0.6);
        base_color = Color::new(
            ((base_color.to_hex() >> 16 & 0xFF) as f32 * (1.0 - aurora_blend) + (aurora_cyan.to_hex() >> 16 & 0xFF) as f32 * aurora_blend) as u8,
            ((base_color.to_hex() >> 8 & 0xFF) as f32 * (1.0 - aurora_blend) + (aurora_cyan.to_hex() >> 8 & 0xFF) as f32 * aurora_blend) as u8,
            ((base_color.to_hex() & 0xFF) as f32 * (1.0 - aurora_blend) + (aurora_cyan.to_hex() & 0xFF) as f32 * aurora_blend) as u8,
        );
    }
    
    let light_dir = Vec3::new(0.4, 0.8, -0.4).normalize();
    let intensity = vertex_normal.dot(&light_dir).max(0.0) * 0.6 + 0.4;
    
    let ice_reflection = if brightness > 0.7 { 1.2 } else { 1.0 };
    
    Color::new(
        (base_color.to_hex() >> 16 & 0xFF) as u8,
        (base_color.to_hex() >> 8 & 0xFF) as u8,
        (base_color.to_hex() & 0xFF) as u8,
    ) * intensity * ice_reflection
}

fn moon_shader(_fragment: &Fragment, uniforms: &Uniforms, vertex_position: &Vec3, vertex_normal: &Vec3) -> Color {
    let _time = uniforms.time as f32 * 0.02;
    
    let zoom = 50.0;
    let x = vertex_position.x;
    let y = vertex_position.y;
    let z = vertex_position.z;
    
    let noise1 = (x * zoom).sin() * (y * zoom).cos() * (z * zoom).sin();
    let noise2 = ((x + 0.3) * zoom * 0.7).cos() * ((y + 0.7) * zoom * 0.9).sin();
    let noise3 = ((z + 0.5) * zoom * 1.3).sin() * ((x - 0.2) * zoom * 0.5).cos();
    let noise4 = ((x * y * zoom * 0.3).sin() + (z * y * zoom * 0.4).cos()) * 0.5;
    
    let combined = noise1 * 0.4 + noise2 * 0.3 + noise3 * 0.2 + noise4 * 0.1;
    
    let dark_gray = Color::new(40, 40, 45);
    let medium_gray = Color::new(70, 70, 75);
    let light_gray = Color::new(100, 100, 105);
    let pale_gray = Color::new(130, 130, 135);
    let bright_gray = Color::new(160, 160, 165);
    
    let crater_pattern = ((x * 30.0).sin() * (z * 30.0).cos() + 1.0) * 0.5;
    
    let base_color = if combined > 0.5 {
        bright_gray
    } else if combined > 0.2 {
        pale_gray
    } else if combined > -0.1 {
        light_gray
    } else if combined > -0.4 {
        medium_gray
    } else {
        dark_gray
    };
    
    let light_dir = Vec3::new(0.5, 0.7, -0.5).normalize();
    let intensity = vertex_normal.dot(&light_dir).max(0.0) * 0.7 + 0.3;
    
    let crater_shadow = if crater_pattern > 0.85 { 0.6 } else { 1.0 };
    
    Color::new(
        (base_color.to_hex() >> 16 & 0xFF) as u8,
        (base_color.to_hex() >> 8 & 0xFF) as u8,
        (base_color.to_hex() & 0xFF) as u8,
    ) * intensity * crater_shadow
}

fn rings_shader(_fragment: &Fragment, uniforms: &Uniforms, vertex_position: &Vec3, vertex_normal: &Vec3) -> Color {
    let time = uniforms.time as f32 * 0.1;
    
    let distance = (vertex_position.x.powi(2) + vertex_position.z.powi(2)).sqrt();
    
    // 4 capas de anillos: bandas principales, bandas secundarias, partículas, densidad
    let bands = ((distance * 50.0).sin() + 1.0) * 0.5;
    let bands2 = ((distance * 80.0 + time).cos() + 1.0) * 0.5;
    let particles = ((vertex_position.x * 100.0).sin() * (vertex_position.z * 100.0).cos() + 1.0) * 0.5;
    let density = ((distance * 30.0).cos() + 1.0) * 0.5;
    
    let combined = bands * 0.4 + bands2 * 0.3 + particles * 0.2 + density * 0.1;
    
    let dark_ring = Color::new(100, 90, 80);
    let medium_ring = Color::new(150, 140, 120);
    let light_ring = Color::new(200, 190, 170);
    let bright_ring = Color::new(230, 220, 200);
    let ice_ring = Color::new(240, 235, 220);
    
    let base_color = if combined > 0.8 {
        ice_ring
    } else if combined > 0.6 {
        bright_ring
    } else if combined > 0.4 {
        light_ring
    } else if combined > 0.25 {
        medium_ring
    } else {
        dark_ring
    };
    
    // Espacios vacíos entre anillos 
    let gap = if distance > 1.3 && distance < 1.4 {
        0.2
    } else if distance > 1.7 && distance < 1.75 {
        0.3
    } else {
        1.0
    };
    
    let light_dir = Vec3::new(0.3, 0.7, -0.6).normalize();
    let intensity = vertex_normal.dot(&light_dir).max(0.0) * 0.5 + 0.5;
    
    Color::new(
        (base_color.to_hex() >> 16 & 0xFF) as u8,
        (base_color.to_hex() >> 8 & 0xFF) as u8,
        (base_color.to_hex() & 0xFF) as u8,
    ) * intensity * gap
}

// Shader simple para la nave - usa sus colores originales con iluminación básica
fn spaceship_shader(fragment: &Fragment, vertex_normal: &Vec3) -> Color {
    // Usar el color del vértice 
    let base_color = fragment.color.clone();
    
    // Iluminación simple direccional
    let light_dir = Vec3::new(0.5, 0.7, -0.3).normalize();
    let intensity = vertex_normal.dot(&light_dir).max(0.2) * 0.8 + 0.2; 
    
    // Aplicar intensidad de luz al color base
    base_color * intensity
}
