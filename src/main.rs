use glium::{
    glutin::{self, dpi::LogicalSize},
    implement_vertex, uniform, Surface,
};
use std::io::Cursor;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let size = LogicalSize::new(400, 300); // (floor(screen_w / pixel_w), floor(screen_h / pixel_h))
    let scaled_size = LogicalSize::new(400 * 3, 300 * 3); // (prev_result * pixel_w, prev_result * pixel_h)
    let wb = glutin::window::WindowBuilder::new()
        .with_title("Conway's Game of Life")
        .with_min_inner_size(size)
        .with_inner_size(scaled_size);
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let image = image::load(
        Cursor::new(include_bytes!("opengl.png")),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();
    let image_dimensions = image.dimensions(); // (floor(screen_w / pixel_w), floor(screen_h / pixel_h))
    let aspect_ratio = image_dimensions.1 as f32 / image_dimensions.0 as f32;
    let image =
        glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::SrgbTexture2d::new(&display, image).unwrap();

    implement_vertex!(Vertex, position, tex_coords);

    let vertex1 = Vertex {
        position: [-1.0, 1.0],
        tex_coords: [0.0, 1.0],
    };
    let vertex2 = Vertex {
        position: [1.0, 1.0],
        tex_coords: [1.0, 1.0],
    };
    let vertex3 = Vertex {
        position: [-1.0, -1.0],
        tex_coords: [0.0, 0.0],
    };
    let vertex4 = Vertex {
        position: [1.0, -1.0],
        tex_coords: [1.0, 0.0],
    };
    let shape = vec![vertex1, vertex2, vertex3, vertex4];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let vertex_shader_src = include_str!("vert.glsl");

    let fragment_shader_src = include_str!("frag.glsl");

    let program =
        glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None)
            .unwrap();

    let uniforms = uniform! {
        matrix: [
            [aspect_ratio, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0f32],
        ],
        tex: texture,
    };

    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                }
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let next_frame_time =
            std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);

        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
    });
}

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}
