use glium::{DisplayBuild, Surface};
use num::traits::Float;
use std::io::prelude::*;
use std::io;
use std::io::Cursor;
use std::fs::File;

trait FullReader: Read {
    fn read_full_string(&mut self) -> io::Result<String> {
        let mut s = String::new();
        let r = self.read_to_string(&mut s);
        r.map(|_| s)
    }

    fn read_full(&mut self) -> io::Result<Vec<u8>> {
        let mut s = Vec::new();
        let r = self.read_to_end(&mut s);
        r.map(|_| s)
    }
}

impl<T> FullReader for T where T: Read {}

#[macro_use]
extern crate glium;
extern crate num;
extern crate image;
extern crate xml;
extern crate chrono;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2]
}

implement_vertex!(Vertex, position, tex_coords);

fn main() {
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    let v1 = Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0] };
    let v2 = Vertex { position: [0.0, 0.5], tex_coords: [0.0, 1.0] };
    let v3 = Vertex { position: [0.5, -0.25], tex_coords: [1.0, 0.0] };
    let shape = vec![v1, v2, v3];

    let image_data = File::open("texture.png").unwrap().read_full().unwrap();
    let image = image::load(Cursor::new(image_data.as_ref()), image::PNG).unwrap();
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let vertex_shader_src = File::open("src/shader.vs").unwrap().read_full_string().unwrap();
    let fragment_shader_src = File::open("src/shader.fs").unwrap().read_full_string().unwrap();

    let program = glium::Program::from_source(&display, vertex_shader_src.as_ref(), fragment_shader_src.as_ref(), None).unwrap();

    let mut t = -0.5;

    loop {
        t += 0.002;
        if t > 0.5 {
            t = -0.5;
        }

        let uniforms = uniform! {
            matrix: [
                [t.cos(), t.sin(), 0.0, 0.0],
                [-t.sin(), t.cos(), 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0]
            ],
            tex: &texture
        };

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniforms, &Default::default()).unwrap();
        target.finish().unwrap();

        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                _ => ()
            }
        }
    }
}
