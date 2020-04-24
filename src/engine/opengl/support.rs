use glutin::{self, PossiblyCurrent};

use std::ffi::CStr;

pub mod gl {
    pub use self::Gles2 as Gl;
    include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

pub struct Gl {
    pub gl: gl::Gl,
}

pub fn load(gl_context: &glutin::Context<PossiblyCurrent>) -> Gl {
    let gl = gl::Gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

    let version = unsafe {
        let data = CStr::from_ptr(gl.GetString(gl::VERSION) as *const _)
            .to_bytes()
            .to_vec();
        String::from_utf8(data).unwrap()
    };

    println!("OpenGL version {}", version);

    unsafe {
        let mut vb = std::mem::zeroed();
        gl.GenBuffers(1, &mut vb);
        gl.BindBuffer(gl::ARRAY_BUFFER, vb);
        gl.BufferData(
            gl::ARRAY_BUFFER,
            (VERTEX_DATA.len() * std::mem::size_of::<f32>())
                as gl::types::GLsizeiptr,
            VERTEX_DATA.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        if gl.BindVertexArray.is_loaded() {
            let mut vao = std::mem::zeroed();
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);
        }

        gl.VertexAttribPointer(
            0,
            2,
            gl::FLOAT,
            0,
            5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            std::ptr::null(),
        );
        gl.VertexAttribPointer(
            1,
            3,
            gl::FLOAT,
            0,
            5 * std::mem::size_of::<f32>() as gl::types::GLsizei,
            (2 * std::mem::size_of::<f32>()) as *const () as *const _,
        );
        gl.EnableVertexAttribArray(0);
        gl.EnableVertexAttribArray(1);
    }

    Gl { gl }
}

impl Gl {
    pub fn draw_frame(&self, color: [f32; 4]) {
        unsafe {
            self.gl.ClearColor(color[0], color[1], color[2], color[3]);
            self.gl.Clear(gl::COLOR_BUFFER_BIT);
            self.gl.DrawArrays(gl::TRIANGLES, 0, 3);
        }
    }
}

#[rustfmt::skip]
static VERTEX_DATA: [f32; 15] = [
    -0.5, -0.5,  1.0,  0.0,  0.0,
     0.0,  0.5,  0.0,  1.0,  0.0,
     0.5, -0.5,  0.0,  0.0,  1.0,
];

