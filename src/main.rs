use glium;
use glium::{implement_vertex, uniform};

use glium::glutin;
use glium::Surface;
use std::time::Instant;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}

implement_vertex!(Vertex, position);

#[allow(dead_code)]
fn print_fps(prev: &mut Instant) {
    let current = Instant::now();
    let d = current.duration_since(*prev);
    println!("FPS: {}", 1_000_000_000 / d.subsec_nanos() as u64);
    *prev = current;
}

fn pos(x: f32, y: f32) -> Vertex {
    Vertex {
        position: [x, y],
    }
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let v1 = pos(-0.5, -0.5);
    let v2 = pos(0.0, 0.5);
    let v3 = pos(0.5, -0.25);
    let shape = vec![v1, v2, v3];

    let vb = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(
        glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = r#"
        #version 140
        in vec2 position;

        uniform float t;

        void main() {
            vec2 pos = position;
            pos.x += t;
            gl_Position = vec4(pos, 0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        out vec4 color;
        void main() {
            color = vec4(1.0, 0.0, 0.0, 1.0);
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
        target.clear_color(1.0, 1.0, 1.0, 1.0);

        let uniform_t = uniform! { t: t };
        target.draw(&vb, &indices, &program, 
                    &uniform_t,
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
