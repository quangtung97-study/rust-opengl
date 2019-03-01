use image;
use std::io::Cursor;

use glium;
use glium::{implement_vertex, uniform};

use glium::glutin;
use glium::Surface;
use std::time::Instant;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

implement_vertex!(Vertex, position, tex_coords);

#[allow(dead_code)]
fn print_fps(prev: &mut Instant) {
    let current = Instant::now();
    let d = current.duration_since(*prev);
    println!("FPS: {}", 1_000_000_000 / d.subsec_nanos() as u64);
    *prev = current;
}

fn pos(x: f32, y: f32, tx: f32, ty: f32) -> Vertex {
    Vertex {
        position: [x, y],
        tex_coords: [tx, ty],
    }
}

#[allow(dead_code)]
fn load_texture(
    path: &str, format: 
    image::ImageFormat, 
    display: &glium::Display) 
    -> glium::texture::Texture2d
{
    let data: Vec<u8> = std::fs::read(path).unwrap();
    let img = image::load(
        Cursor::new(data), format).unwrap().to_rgba();
    let dims = img.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(
        &img.into_raw(), dims);
    glium::texture::Texture2d::new(display, image).unwrap()
}


fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let v1 = pos(-0.5, -0.5, 0.0, 0.0);
    let v2 = pos(0.0, 0.5, 0.0, 1.0);
    let v3 = pos(0.5, -0.25, 1.0, 0.0);
    let v4 = pos(0.5, 0.6, 1.0, 1.0);
    let shape = vec![v1, v2, v3, v4];

    let vb = glium::VertexBuffer::new(&display, &shape).unwrap();
    let index_vec: Vec<u16> = vec![0, 1, 2, 1, 2, 3];
    let indices = glium::IndexBuffer::new(
        &display, glium::index::PrimitiveType::TrianglesList,
        index_vec.as_slice()).unwrap();
                                          

    let texture = load_texture("cute.png", image::PNG, &display);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;
        in vec2 tex_coords;

        out vec2 v_tex_coords;

        uniform mat4 matrix;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix * vec4(position, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec2 v_tex_coords;
        out vec4 color;

        uniform sampler2D tex;

        void main() {
            color = texture(tex, v_tex_coords);
        }
    "#;

    let program = glium::Program::from_source(
        &display, vertex_shader_src, fragment_shader_src, None).unwrap();

    #[allow(unused_variables, unused_mut)]
    let mut prev_instant = Instant::now();
    let mut t: f32 = -0.5;
    let mut closed = false;
    while !closed {
        t += 0.0002;
        if t > 0.5 {
            t = -0.5;
        }

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        let uniform_matrix = uniform! {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [t  , 0.0, 0.0, 1.0f32],
            ],
            tex: &texture,
        };
        target.draw(&vb, &indices, &program, 
                    &uniform_matrix,
                    &Default::default()).unwrap();

        target.finish().unwrap();

        events_loop.poll_events(|e| {
            match e {
                glutin::Event::WindowEvent { event, .. } => match event {
                    glutin::WindowEvent::CloseRequested => closed = true,
                    _ => (),
                },
                _ => (),
            }
        });

        // print_fps(&mut prev_instant);
    }
}
