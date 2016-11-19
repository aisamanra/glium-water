#[macro_use]
extern crate glium;
extern crate image;

// We need to have the relevant traits in scope
use glium::{DisplayBuild, Surface};

// Unfortunately, we can't just borrow an existing point implementation, so
// here's a basic one:
#[derive(Copy, Clone)]
struct V {
    pt: [f32; 2],
}
implement_vertex!(V, pt);

// This embeds the image source directly in the binary rather than requiring
// it at runtime, although it does end up parsing the texture at runtime
// anyway.
fn get_texture<'a>() -> glium::texture::RawImage2d<'a, u8> {
    use std::io::Cursor;
    let image = image::load(
        Cursor::new(&include_bytes!("../data/mask.jpeg")[..]),
        image::JPEG).unwrap().to_rgba();
    let dims = image.dimensions();
    glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), dims)
}

fn main() {
    // We let Glutin handle building a window for us
    let display = glium::glutin::WindowBuilder::new().build_glium().unwrap();

    // Grab the texture data and wrap it in a Glium texture
    let tex = glium::texture::Texture2d::new(&display, get_texture()).unwrap();

    // We need to load up our shaders, which we do by including their source
    // files relative to this source file
    let program = glium::Program::from_source(
        &display,
        include_str!("vertex.glsl"),
        include_str!("shader.glsl"),
        None).unwrap();

    // We sadly need some polys to draw to---we can't just a plain fragment
    // shader, AFAIK---so here's the points for two triangles that cover
    // the whole screen space
    let vx = vec![ V { pt: [-1.0,  1.0] },
                   V { pt: [ 1.0,  1.0] },
                   V { pt: [-1.0, -1.0] },

                   V { pt: [-1.0, -1.0] },
                   V { pt: [ 1.0,  1.0] },
                   V { pt: [ 1.0, -1.0] },
                   ];
    // ...and a vertex buffer that contains them
    let vbuf = glium::VertexBuffer::new(&display, &vx).unwrap();
    // ...and the indices for those triangles, too
    let ibuf = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // finally, we need to keep track of the current time
    let mut time = 0.0f32;

    // for every frame...
    loop {
        // grab a drawable object
        let mut target = display.draw();
        // find the current dimensions of the window
        let (x, y) = target.get_dimensions();

        // draw the polys with the supplied shaders, passing the texture,
        // the time, and the current window dimensions as uniforms
        target.draw(&vbuf,
                    &ibuf,
                    &program,
                    &uniform! {
                        time: time,
                        water: &tex,
                        dims: (x as f32, y as f32),
                    },
                    &Default::default()).unwrap();

        // and finalize the frame
        target.finish().unwrap();

        // return if we try to close the window or press 'Esc' or 'q'
        for ev in display.poll_events() {
            match ev {
                glium::glutin::Event::Closed => return,
                glium::glutin::Event::KeyboardInput(_, 9, _) => return,
                glium::glutin::Event::KeyboardInput(_, 24, _) => return,
                _ => (),
            }
        }

        // otherwise, increment the timer!
        time += 0.01;
    }
}
