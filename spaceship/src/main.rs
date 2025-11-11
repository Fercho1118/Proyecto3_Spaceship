use nalgebra_glm::{Vec3, Mat4, look_at, perspective};
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

use framebuffer::Framebuffer;
use vertex::Vertex;
use obj::Obj;
use triangle::triangle;
use shaders::vertex_shader;
use sphere::create_sphere;
use ring::create_ring;
use fragment_shader::ShaderType;


pub struct Uniforms {
    pub model_matrix: Mat4,
    pub view_matrix: Mat4,
    pub projection_matrix: Mat4,
    pub viewport_matrix: Mat4,
    pub time: u32,
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
        "Shader Lab - Celestial Bodies",
        window_width,
        window_height,
        WindowOptions::default(),
    )
    .unwrap();

    window.set_position(500, 500);
    window.update();

    framebuffer.set_background_color(0x000000);

    let sphere = create_sphere(1.0, 50, 50);
    let moon_sphere = create_sphere(0.3, 30, 30);
    let ring = create_ring(1.3, 2.0, 100);

    let mut camera_position = Vec3::new(0.0, 0.0, 5.0);
    let mut rotation = Vec3::new(0.0, 0.0, 0.0);
    let mut last_mouse_pos: Option<(f32, f32)> = None;
    let mut auto_rotate = true;
    let mut current_scene = 1;
    let mut time = 0u32;

    let projection_matrix = create_perspective_matrix(window_width as f32, window_height as f32);
    let viewport_matrix = create_viewport_matrix(framebuffer_width as f32, framebuffer_height as f32);

    println!("Controles:");
    println!("- 1: Sol (Estrella)");
    println!("- 2: Planeta Terrestre con Luna");
    println!("- 3: Gigante Gaseoso con Anillos");
    println!("- 4: Planeta Helado");
    println!("- 5: Luna (solo)");
    println!("- WASD: Mover cámara");
    println!("- QE: Mover cámara arriba/abajo");
    println!("- Mouse: Rotar planeta");
    println!("- ESPACIO: Rotación automática");
    println!("- ESC: Salir");

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        if window.is_key_pressed(Key::Key1, minifb::KeyRepeat::No) {
            current_scene = 1;
            println!("Escena: Sol");
        }
        if window.is_key_pressed(Key::Key2, minifb::KeyRepeat::No) {
            current_scene = 2;
            println!("Escena: Planeta Terrestre con Luna");
        }
        if window.is_key_pressed(Key::Key3, minifb::KeyRepeat::No) {
            current_scene = 3;
            println!("Escena: Gigante Gaseoso con Anillos");
        }
        if window.is_key_pressed(Key::Key4, minifb::KeyRepeat::No) {
            current_scene = 4;
            println!("Escena: Planeta Helado");
        }
        if window.is_key_pressed(Key::Key5, minifb::KeyRepeat::No) {
            current_scene = 5;
            println!("Escena: Luna");
        }

        if window.is_key_pressed(Key::Space, minifb::KeyRepeat::No) {
            auto_rotate = !auto_rotate;
        }

        if window.is_key_down(Key::W) {
            camera_position.z -= 0.1;
        }
        if window.is_key_down(Key::S) {
            camera_position.z += 0.1;
        }
        if window.is_key_down(Key::A) {
            camera_position.x -= 0.1;
        }
        if window.is_key_down(Key::D) {
            camera_position.x += 0.1;
        }
        if window.is_key_down(Key::Q) {
            camera_position.y += 0.1;
        }
        if window.is_key_down(Key::E) {
            camera_position.y -= 0.1;
        }
        
        if window.get_mouse_down(minifb::MouseButton::Left) {
            if let Some(pos) = window.get_mouse_pos(minifb::MouseMode::Discard) {
                if let Some(last_pos) = last_mouse_pos {
                    let dx = pos.0 - last_pos.0;
                    let dy = pos.1 - last_pos.1;
                    
                    rotation.y -= dx * 0.01;
                    rotation.x -= dy * 0.01;
                    auto_rotate = false;
                }
                last_mouse_pos = Some(pos);
            }
        } else {
            last_mouse_pos = None;
        }
        
        if auto_rotate {
            rotation.y += 0.01;
        }

        time += 1;

        framebuffer.clear();

        let view_matrix = create_view_matrix(camera_position, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        
        match current_scene {
            1 => {
                let model_matrix = create_model_matrix(Vec3::new(0.0, 0.0, 0.0), 1.5, rotation);
                let uniforms = Uniforms { 
                    model_matrix, 
                    view_matrix,
                    projection_matrix,
                    viewport_matrix,
                    time 
                };
                render(&mut framebuffer, &uniforms, &sphere, &ShaderType::Sun);
            },
            2 => {
                // Planeta terrestre
                let model_matrix = create_model_matrix(Vec3::new(0.0, 0.0, 0.0), 1.5, rotation);
                let uniforms = Uniforms { 
                    model_matrix, 
                    view_matrix,
                    projection_matrix,
                    viewport_matrix,
                    time 
                };
                render(&mut framebuffer, &uniforms, &sphere, &ShaderType::EarthLike);
                
                // Luna orbitando
                let moon_angle = time as f32 * 0.02;
                let moon_distance = 2.5;
                let moon_x = moon_angle.cos() * moon_distance;
                let moon_z = moon_angle.sin() * moon_distance;
                
                let moon_model_matrix = create_model_matrix(
                    Vec3::new(moon_x, 0.0, moon_z), 
                    0.3, 
                    Vec3::new(0.0, time as f32 * 0.01, 0.0)
                );
                let moon_uniforms = Uniforms {
                    model_matrix: moon_model_matrix,
                    view_matrix,
                    projection_matrix,
                    viewport_matrix,
                    time
                };
                render(&mut framebuffer, &moon_uniforms, &moon_sphere, &ShaderType::Moon);
            },
            3 => {
                // Gigante gaseoso
                let model_matrix = create_model_matrix(Vec3::new(0.0, 0.0, 0.0), 1.5, rotation);
                let uniforms = Uniforms { 
                    model_matrix, 
                    view_matrix,
                    projection_matrix,
                    viewport_matrix,
                    time 
                };
                render(&mut framebuffer, &uniforms, &sphere, &ShaderType::GasGiant);
                
                // Anillos inclinados
                let ring_rotation = Vec3::new(PI / 6.0, rotation.y, 0.0);
                let ring_model_matrix = create_model_matrix(Vec3::new(0.0, 0.0, 0.0), 1.5, ring_rotation);
                let ring_uniforms = Uniforms {
                    model_matrix: ring_model_matrix,
                    view_matrix,
                    projection_matrix,
                    viewport_matrix,
                    time
                };
                render(&mut framebuffer, &ring_uniforms, &ring, &ShaderType::Rings);
            },
            4 => {
                let model_matrix = create_model_matrix(Vec3::new(0.0, 0.0, 0.0), 1.5, rotation);
                let uniforms = Uniforms { 
                    model_matrix, 
                    view_matrix,
                    projection_matrix,
                    viewport_matrix,
                    time 
                };
                render(&mut framebuffer, &uniforms, &sphere, &ShaderType::IcePlanet);
            },
            5 => {
                let model_matrix = create_model_matrix(Vec3::new(0.0, 0.0, 0.0), 2.0, rotation);
                let uniforms = Uniforms { 
                    model_matrix, 
                    view_matrix,
                    projection_matrix,
                    viewport_matrix,
                    time 
                };
                render(&mut framebuffer, &uniforms, &sphere, &ShaderType::Moon);
            },
            _ => {}
        }

        window
            .update_with_buffer(&framebuffer.buffer, framebuffer_width, framebuffer_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
