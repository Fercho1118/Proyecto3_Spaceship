use nalgebra_glm::{Vec2, Vec3, Vec4, Mat4, look_at, perspective};
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use std::f32::consts::PI;

mod framebuffer;
mod triangle;
mod line;
mod vertex;
mod obj;
mod color;
mod fragment;
mod shaders;
mod sphere;
mod fragment_shader;
mod ring;
mod skybox;

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use triangle::triangle;
use shaders::vertex_shader;
use sphere::create_sphere;
use ring::create_ring;
use fragment_shader::ShaderType;
use line::line;
use color::Color;
use skybox::Skybox;


pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4,
    pub time: u32,
}

// Estructura para representar un cuerpo celeste
struct CelestialBody {
    position: Vec3,
    scale: f32,
    orbit_radius: f32,
    orbit_speed: f32,
    rotation_speed: f32,
    shader_type: ShaderType,
    current_orbit_angle: f32,
    current_rotation_angle: f32,
    has_rings: bool,
    moon: Option<Box<CelestialBody>>,
}

struct Spaceship {
    position: Vec3,
    rotation: Vec3,
    velocity: Vec3,
    scale: f32,
    speed: f32,
    rotation_speed: f32,
    collision_cooldown: f32,
}

impl CelestialBody {
    fn new(
        orbit_radius: f32,
        scale: f32,
        orbit_speed: f32,
        rotation_speed: f32,
        shader_type: ShaderType,
    ) -> Self {
        Self {
            position: Vec3::new(orbit_radius, 0.0, 0.0),
            scale,
            orbit_radius,
            orbit_speed,
            rotation_speed,
            shader_type,
            current_orbit_angle: 0.0,
            current_rotation_angle: 0.0,
            has_rings: false,
            moon: None,
        }
    }

    fn with_rings(mut self) -> Self {
        self.has_rings = true;
        self
    }

    fn with_moon(mut self, moon: CelestialBody) -> Self {
        self.moon = Some(Box::new(moon));
        self
    }

    fn update(&mut self, delta_time: f32) {
        self.current_orbit_angle += self.orbit_speed * delta_time;
        if self.current_orbit_angle > 2.0 * PI {
            self.current_orbit_angle -= 2.0 * PI;
        }

        self.position.x = self.orbit_radius * self.current_orbit_angle.cos();
        self.position.z = self.orbit_radius * self.current_orbit_angle.sin();

        self.current_rotation_angle += self.rotation_speed * delta_time;
        if self.current_rotation_angle > 2.0 * PI {
            self.current_rotation_angle -= 2.0 * PI;
        }

        if let Some(moon) = &mut self.moon {
            moon.update(delta_time);
            moon.position.x = self.position.x + moon.orbit_radius * moon.current_orbit_angle.cos();
            moon.position.z = self.position.z + moon.orbit_radius * moon.current_orbit_angle.sin();
        }
    }
    
    fn get_collision_radius(&self) -> f32 {
        self.scale
    }
}

struct SolarSystem {
    sun: CelestialBody,
    planets: Vec<CelestialBody>,
}

impl SolarSystem {
    // Detecta colisiones y empuja la nave fuera de los cuerpos celestes
    fn check_and_resolve_collision(&self, spaceship_pos: Vec3, spaceship_radius: f32) -> Option<(String, Vec3)> {
        let direction_from_sun = spaceship_pos - self.sun.position;
        let distance_to_sun = direction_from_sun.magnitude();
        let min_distance_sun = self.sun.get_collision_radius() + spaceship_radius;
        
        if distance_to_sun < min_distance_sun {
            let push_direction = direction_from_sun.normalize();
            let new_position = self.sun.position + push_direction * min_distance_sun;
            return Some(("Sol".to_string(), new_position));
        }
        
        for (i, planet) in self.planets.iter().enumerate() {
            let direction_from_planet = spaceship_pos - planet.position;
            let distance = direction_from_planet.magnitude();
            let min_distance = planet.get_collision_radius() + spaceship_radius;
            
            if distance < min_distance {
                let push_direction = direction_from_planet.normalize();
                let new_position = planet.position + push_direction * min_distance;
                let planet_names = ["Planeta Tierra", "Planeta Gaseoso", "Planeta Helado"];
                return Some((planet_names[i].to_string(), new_position));
            }
            
            if let Some(moon) = &planet.moon {
                let direction_from_moon = spaceship_pos - moon.position;
                let distance_to_moon = direction_from_moon.magnitude();
                let min_distance_moon = moon.get_collision_radius() + spaceship_radius;
                
                if distance_to_moon < min_distance_moon {
                    let push_direction = direction_from_moon.normalize();
                    let new_position = moon.position + push_direction * min_distance_moon;
                    return Some(("Luna".to_string(), new_position));
                }
            }
        }
        
        None
    }
}

fn draw_orbit(
    framebuffer: &mut Framebuffer,
    center: Vec3,
    radius: f32,
    segments: u32,
    color: Color,
    view_matrix: &Mat4,
    projection_matrix: &Mat4,
    viewport_matrix: &Mat4,
) {
    let mvp = projection_matrix * view_matrix;
    
    for i in 0..segments {
        let angle1 = (i as f32 / segments as f32) * 2.0 * PI;
        let angle2 = ((i + 1) as f32 / segments as f32) * 2.0 * PI;
        
        let p1 = Vec3::new(
            center.x + radius * angle1.cos(),
            center.y,
            center.z + radius * angle1.sin()
        );
        let p2 = Vec3::new(
            center.x + radius * angle2.cos(),
            center.y,
            center.z + radius * angle2.sin()
        );
        
        let p1_clip = mvp * Vec4::new(p1.x, p1.y, p1.z, 1.0);
        let p2_clip = mvp * Vec4::new(p2.x, p2.y, p2.z, 1.0);
        
        if p1_clip.w.abs() < 0.001 || p2_clip.w.abs() < 0.001 {
            continue;
        }
        
        let p1_ndc = Vec3::new(
            p1_clip.x / p1_clip.w,
            p1_clip.y / p1_clip.w,
            p1_clip.z / p1_clip.w
        );
        let p2_ndc = Vec3::new(
            p2_clip.x / p2_clip.w,
            p2_clip.y / p2_clip.w,
            p2_clip.z / p2_clip.w
        );
        
        if p1_ndc.x.abs() > 2.0 || p1_ndc.y.abs() > 2.0 || 
           p2_ndc.x.abs() > 2.0 || p2_ndc.y.abs() > 2.0 ||
           p1_clip.w < 0.0 || p2_clip.w < 0.0 {
            continue;
        }
        
        let viewport = viewport_matrix;
        let p1_screen = viewport * Vec4::new(p1_ndc.x, p1_ndc.y, p1_ndc.z, 1.0);
        let p2_screen = viewport * Vec4::new(p2_ndc.x, p2_ndc.y, p2_ndc.z, 1.0);
        
        if p1_screen.x.is_nan() || p1_screen.y.is_nan() || 
           p2_screen.x.is_nan() || p2_screen.y.is_nan() {
            continue;
        }
        
        let v1 = Vertex {
            position: p1,
            normal: Vec3::new(0.0, 1.0, 0.0),
            tex_coords: Vec2::new(0.0, 0.0),
            color: color.clone(),
            transformed_position: Vec3::new(p1_screen.x, p1_screen.y, p1_screen.z),
            transformed_normal: Vec3::new(0.0, 1.0, 0.0),
        };
        let v2 = Vertex {
            position: p2,
            normal: Vec3::new(0.0, 1.0, 0.0),
            tex_coords: Vec2::new(0.0, 0.0),
            color: color.clone(),
            transformed_position: Vec3::new(p2_screen.x, p2_screen.y, p2_screen.z),
            transformed_normal: Vec3::new(0.0, 1.0, 0.0),
        };
        
        let fragments = line(&v1, &v2);
        for fragment in fragments {
            if fragment.position.x < 0.0 || fragment.position.y < 0.0 ||
               fragment.position.x.is_nan() || fragment.position.y.is_nan() {
                continue;
            }
            
            let x = fragment.position.x as usize;
            let y = fragment.position.y as usize;
            if x < framebuffer.width && y < framebuffer.height {
                framebuffer.set_current_color(color.to_hex());
                framebuffer.point(x, y, fragment.depth);
            }
        }
    }
}

fn create_model_matrix(translation: Vec3, scale: f32, rotation: Vec3) -> Mat4 {
    let (sin_x, cos_x) = rotation.x.sin_cos();
    let (sin_y, cos_y) = rotation.y.sin_cos();
    let (sin_z, cos_z) = rotation.z.sin_cos();

    let rotation_matrix_x = Mat4::new(
        1.0,  0.0,    0.0,   0.0,
        0.0,  cos_x, -sin_x, 0.0,
        0.0,  sin_x,  cos_x, 0.0,
        0.0,  0.0,    0.0,   1.0,
    );

    let rotation_matrix_y = Mat4::new(
        cos_y,  0.0,  sin_y, 0.0,
        0.0,    1.0,  0.0,   0.0,
        -sin_y, 0.0,  cos_y, 0.0,
        0.0,    0.0,  0.0,   1.0,
    );

    let rotation_matrix_z = Mat4::new(
        cos_z, -sin_z, 0.0, 0.0,
        sin_z,  cos_z, 0.0, 0.0,
        0.0,    0.0,  1.0, 0.0,
        0.0,    0.0,  0.0, 1.0,
    );

    let rotation_matrix = rotation_matrix_z * rotation_matrix_y * rotation_matrix_x;

    let transform_matrix = Mat4::new(
        scale, 0.0,   0.0,   translation.x,
        0.0,   scale, 0.0,   translation.y,
        0.0,   0.0,   scale, translation.z,
        0.0,   0.0,   0.0,   1.0,
    );

    transform_matrix * rotation_matrix
}

fn create_view_matrix(eye: Vec3, center: Vec3, up: Vec3) -> Mat4 {
    look_at(&eye, &center, &up)
}

fn create_perspective_matrix(window_width: f32, window_height: f32) -> Mat4 {
    let fov = 45.0 * PI / 180.0;
    let aspect_ratio = window_width / window_height;
    let near = 0.1;
    let far = 100.0;

    perspective(fov, aspect_ratio, near, far)
}

fn create_viewport_matrix(width: f32, height: f32) -> Mat4 {
    Mat4::new(
        width / 2.0, 0.0, 0.0, width / 2.0,
        0.0, -height / 2.0, 0.0, height / 2.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0
    )
}

fn render(framebuffer: &mut Framebuffer, uniforms: &Uniforms, vertex_array: &[Vertex], shader_type: &ShaderType) {
    let mut transformed_vertices = Vec::with_capacity(vertex_array.len());
    for vertex in vertex_array {
        let transformed = vertex_shader(vertex, uniforms);
        transformed_vertices.push(transformed);
    }

    let mut triangles = Vec::new();
    for i in (0..transformed_vertices.len()).step_by(3) {
        if i + 2 < transformed_vertices.len() {
            triangles.push([
                transformed_vertices[i].clone(),
                transformed_vertices[i + 1].clone(),
                transformed_vertices[i + 2].clone(),
            ]);
        }
    }

    let mut fragments = Vec::new();
    for tri in &triangles {
        fragments.extend(triangle(&tri[0], &tri[1], &tri[2], uniforms, shader_type));
    }

    for fragment in fragments {
        let x = fragment.position.x as usize;
        let y = fragment.position.y as usize;
        if x < framebuffer.width && y < framebuffer.height {
            let color = fragment.color.to_hex();
            framebuffer.set_current_color(color);
            framebuffer.point(x, y, fragment.depth);
        }
    }
}

fn main() {
    let window_width = 800;
    let window_height = 800;
    let framebuffer_width = 800;
    let framebuffer_height = 800;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);
    let mut window = Window::new(
        "Sistema Solar - Software Renderer",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x000000);

    let skybox = Skybox::new(800);
    let sphere = create_sphere(1.0, 50, 50);
    let ring = create_ring(1.3, 2.0, 100);
    
    let spaceship_obj = Obj::load("assets/Jett.obj").expect("Error cargando modelo de nave");
    let spaceship_vertices = spaceship_obj.get_vertex_array();
    
    let mut spaceship = Spaceship {
        position: Vec3::new(0.0, 8.0, 35.0),
        rotation: Vec3::new(0.0, PI, 0.0),
        velocity: Vec3::new(0.0, 0.0, 0.0),
        scale: 0.08,
        speed: 0.15,
        rotation_speed: 0.03,
        collision_cooldown: 0.0,
    };

    // Crear sistema solar
    let mut solar_system = SolarSystem {
        sun: CelestialBody {
            position: Vec3::new(0.0, 0.0, 0.0),
            scale: 2.0,
            orbit_radius: 0.0,
            orbit_speed: 0.0,
            rotation_speed: 0.1,
            shader_type: ShaderType::Sun,
            current_orbit_angle: 0.0,
            current_rotation_angle: 0.0,
            has_rings: false,
            moon: None,
        },
        planets: vec![
            CelestialBody::new(8.0, 0.8, 0.5, 1.0, ShaderType::EarthLike)
                .with_moon(CelestialBody::new(1.5, 0.3, 2.0, 0.5, ShaderType::Moon)),
            CelestialBody::new(15.0, 1.5, 0.3, 0.8, ShaderType::GasGiant)
                .with_rings(),
            CelestialBody::new(22.0, 0.6, 0.2, 0.9, ShaderType::IcePlanet),
        ],
    };

    let mut time = 0u32;
    let delta_time = 0.016;

    let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
    let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

    println!("=== SISTEMA SOLAR - CONTROLES ===");
    println!("Controles de la Nave:");
    println!("  W/S: Acelerar/Frenar");
    println!("  A/D: Rotar izquierda/derecha");
    println!("  Flechas Arriba/Abajo: Inclinar nave");
    println!("  Q/E: Subir/Bajar");
    println!("Otros:");
    println!("  ESPACIO: Pausar/Reanudar órbitas planetarias");
    println!("  O: Mostrar/Ocultar órbitas");
    println!("  ESC: Salir");
    println!("==================================");

    let mut paused = false;
    let mut show_orbits = true;

    while window.is_open() {
        if window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No) {
            break;
        }

        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            paused = !paused;
            println!("{}", if paused { "Sistema pausado" } else { "Sistema activo" });
        }

        if window.is_key_pressed(Key::O, minifb::KeyRepeat::No) {
            show_orbits = !show_orbits;
            println!("{}", if show_orbits { "Órbitas visibles" } else { "Órbitas ocultas" });
        }

        let yaw = spaceship.rotation.y;
        let pitch = spaceship.rotation.x;
        
        if window.is_key_down(Key::W) {
            spaceship.position.x += yaw.sin() * spaceship.speed;
            spaceship.position.z += yaw.cos() * spaceship.speed;
        }
        if window.is_key_down(Key::S) {
            spaceship.position.x -= yaw.sin() * spaceship.speed;
            spaceship.position.z -= yaw.cos() * spaceship.speed;
        }
        
        if window.is_key_down(Key::A) {
            spaceship.rotation.y += spaceship.rotation_speed;
        }
        if window.is_key_down(Key::D) {
            spaceship.rotation.y -= spaceship.rotation_speed;
        }
        
        if window.is_key_down(Key::Up) {
            spaceship.rotation.x += spaceship.rotation_speed * 0.5;
            spaceship.rotation.x = spaceship.rotation.x.min(PI / 6.0);
        }
        if window.is_key_down(Key::Down) {
            spaceship.rotation.x -= spaceship.rotation_speed * 0.5;
            spaceship.rotation.x = spaceship.rotation.x.max(-PI / 6.0);
        }
        
        if window.is_key_down(Key::Q) {
            spaceship.position.y += spaceship.speed;
        }
        if window.is_key_down(Key::E) {
            spaceship.position.y -= spaceship.speed;
        }

        if !paused {
            solar_system.sun.update(delta_time);
            for planet in &mut solar_system.planets {
                planet.update(delta_time);
            }
        }
        
        let camera_distance = 2.5;
        let camera_height = 0.8;
        let camera_position = Vec3::new(
            spaceship.position.x - yaw.sin() * camera_distance,
            spaceship.position.y + camera_height,
            spaceship.position.z - yaw.cos() * camera_distance
        );
        
        if spaceship.collision_cooldown > 0.0 {
            spaceship.collision_cooldown -= delta_time;
        }
        
        let spaceship_collision_radius = spaceship.scale * 5.0;
        if let Some((collision_name, corrected_position)) = solar_system.check_and_resolve_collision(spaceship.position, spaceship_collision_radius) {
            spaceship.position = corrected_position;
            
            if spaceship.collision_cooldown <= 0.0 {
                println!("¡COLISIÓN con {}!", collision_name);
                spaceship.collision_cooldown = 1.0;
            }
        }

        time += 1;

        framebuffer.clear();
        
        skybox.render(&mut framebuffer);

        let view_matrix = create_view_matrix(camera_position, spaceship.position, Vec3::new(0.0, 1.0, 0.0));

        if show_orbits {
            draw_orbit(
                &mut framebuffer,
                Vec3::new(0.0, 0.0, 0.0),
                8.0,
                100,
                Color::new(0, 255, 100),
                &view_matrix,
                &projection_matrix,
                &viewport_matrix
            );

            draw_orbit(
                &mut framebuffer,
                Vec3::new(0.0, 0.0, 0.0),
                15.0,
                120,
                Color::new(200, 100, 255),
                &view_matrix,
                &projection_matrix,
                &viewport_matrix
            );

            draw_orbit(
                &mut framebuffer,
                Vec3::new(0.0, 0.0, 0.0),
                22.0,
                140,
                Color::new(100, 200, 255),
                &view_matrix,
                &projection_matrix,
                &viewport_matrix
            );
        }

        {
            let sun_model_matrix = create_model_matrix(
                solar_system.sun.position,
                solar_system.sun.scale,
                Vec3::new(0.0, solar_system.sun.current_rotation_angle, 0.0)
            );
            let sun_uniforms = Uniforms {
                model_matrix: sun_model_matrix,
                view_matrix,
                projection_matrix,
                viewport_matrix,
                time,
            };
            render(&mut framebuffer, &sun_uniforms, &sphere, &solar_system.sun.shader_type);
        }

        for planet in &solar_system.planets {
                let planet_model_matrix = create_model_matrix(
                    planet.position,
                    planet.scale,
                    Vec3::new(0.0, planet.current_rotation_angle, 0.0)
                );
                let planet_uniforms = Uniforms {
                    model_matrix: planet_model_matrix,
                    view_matrix,
                    projection_matrix,
                    viewport_matrix,
                    time,
                };
                render(&mut framebuffer, &planet_uniforms, &sphere, &planet.shader_type);

                if planet.has_rings {
                    let ring_rotation = Vec3::new(PI / 6.0, planet.current_rotation_angle, 0.0);
                    let ring_model_matrix = create_model_matrix(planet.position, planet.scale, ring_rotation);
                    let ring_uniforms = Uniforms {
                        model_matrix: ring_model_matrix,
                        view_matrix,
                        projection_matrix,
                        viewport_matrix,
                        time,
                    };
                    render(&mut framebuffer, &ring_uniforms, &ring, &ShaderType::Rings);
                }

            if let Some(moon) = &planet.moon {
                    let moon_model_matrix = create_model_matrix(
                        moon.position,
                        moon.scale,
                        Vec3::new(0.0, moon.current_rotation_angle, 0.0)
                    );
                    let moon_uniforms = Uniforms {
                        model_matrix: moon_model_matrix,
                        view_matrix,
                        projection_matrix,
                        viewport_matrix,
                        time,
                    };
                    render(&mut framebuffer, &moon_uniforms, &sphere, &moon.shader_type);
                }
        }

        // Corrección de orientación del modelo 3D
        let spaceship_corrected_rotation = Vec3::new(
            spaceship.rotation.x + PI,
            spaceship.rotation.y,
            spaceship.rotation.z
        );
        let spaceship_model_matrix = create_model_matrix(
            spaceship.position,
            spaceship.scale,
            spaceship_corrected_rotation
        );
        let spaceship_uniforms = Uniforms {
            model_matrix: spaceship_model_matrix,
            view_matrix,
            projection_matrix,
            viewport_matrix,
            time,
        };
        render(&mut framebuffer, &spaceship_uniforms, &spaceship_vertices, &ShaderType::Spaceship);

        // Efecto visual de colisión
        if spaceship.collision_cooldown > 0.0 {
            let flash_intensity = (spaceship.collision_cooldown * 127.5) as u8;
            let red_color = ((flash_intensity as u32) << 16) | 0x000000;
            
            let border_thickness = 10;
            for y in 0..framebuffer_height {
                for x in 0..border_thickness {
                    framebuffer.set_current_color(red_color);
                    framebuffer.point(x, y, 0.0);
                    framebuffer.point(framebuffer_width - 1 - x, y, 0.0);
                }
            }
            for x in 0..framebuffer_width {
                for y in 0..border_thickness {
                    framebuffer.set_current_color(red_color);
                    framebuffer.point(x, y, 0.0);
                    framebuffer.point(x, framebuffer_height - 1 - y, 0.0);
                }
            }
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
