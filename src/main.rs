use std::{error::Error, path::Path, mem::size_of};
use alice_gl::{
    buffer::{
        core::{Bind, Buffer, IdObject},
        generator::Generator,
        vao::Vao,
        vbo::Vbo,
        ebo::Ebo
    },
    constant::{Boolean, Type, Usage},
    shader::{
        core::{DrawMode, Shader}, fragment::FragmentShader, program::Program, vertex::VertexShader
    },
    window::*
};
use sdl3::{event::Event, video::GLProfile};

fn main() -> Result<(), Box<dyn Error>> {
    let sdl3 = sdl3::init()?;
    let video_system = sdl3.video()?;
    let window = video_system.window("GL Test", 800, 600).opengl().build()?;

    let gl_attr = video_system.gl_attr();
    gl_attr.set_context_version(4, 2);
    gl_attr.set_context_profile(GLProfile::Core);

    let gl_context = window.gl_create_context()?;
    window.gl_make_current(&gl_context)?;
    alice_gl::load_with(|s| video_system.gl_get_proc_address(s).unwrap() as *const _);

    viewport(0, 0, 800, 600);
    clearcolor(0.5, 0.5, 1.0, 1.0);
    let mut geneater = Generator::new();

    let vertex = [
        -0.8, -0.5, 0.0, 0.2, 0.0, 0.3,
        0.8, -0.5, 0.0, 0.5, 0.0, 0.8,
        -0.8, 0.5, 0.0, 0.5, 0.0, 0.8,
        0.8, 0.5, 0.0, 0.2, 0.0, 0.3,
    ];

    let mut vbo = Vbo::create(&mut geneater);
    vbo.bind();
    vbo.upload(
        &vertex,
        Usage::StaticDraw
    );

    let mut vao = Vao::create(&mut geneater);
    vao.bind();
    Vao::enable(0);
    vao.upload(0, 3, Type::Float, Boolean::False, (6*size_of::<f32>()) as i32, None);
    Vao::enable(1);
    vao.upload(1, 3, Type::Float, Boolean::False, (6*size_of::<f32>()) as i32, Some(3*size_of::<f32>()));

    let vertex_shader = VertexShader::compile_from("TestVertexShader", Path::new("./src/test.vert"));
    let fragment_shader = FragmentShader::compile_from("TestFragmentShader", Path::new("./src/test.frag"));
    let mut shader = Program::link_from(&vertex_shader, &fragment_shader);
    shader.use_program();

    let mut ebo = Ebo::create(&mut geneater);
    ebo.bind();
    let indices = [
        0, 1, 2,
        1, 2, 3
    ];
    ebo.upload(&indices, Usage::StaticDraw);

    let mut event_pump = sdl3.event_pump()?;
    'Running: loop {
        clear();

        for event in event_pump.poll_iter() {
            match event {

                Event::Quit { .. } => break 'Running,

                _ => {}
            }
        }

        shader.draw_elements(DrawMode::Triangles, 6, Type::UInt, None);
        window.gl_swap_window();
    }

    Ok(())
}
