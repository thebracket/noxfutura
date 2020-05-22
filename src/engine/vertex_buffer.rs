use crate::opengl::*;
use std::mem;
use std::os::raw::c_void;

#[derive(Debug)]
pub struct VertexArray {
    pub vertex_buffer: Vec<f32>,
    vao: u32,
    vbo: u32,
    element_size: i32
}

pub struct VertexArrayEntry {
    pub index: u32,
    pub size: i32,
}

impl VertexArray {
    pub fn float_builder(
        gl: &Gl,
        entries: &[VertexArrayEntry],
        vertex_capacity: usize
    ) -> Self {
        let mut buffer = VertexArray {
            vertex_buffer: Vec::with_capacity(vertex_capacity),
            vao: 0,
            vbo: 0,
            element_size: entries.iter().map(|e| e.size).sum()
        };

        gl_error(gl);
        unsafe {
            gl.GenVertexArrays(1, &mut buffer.vao);

            gl.GenBuffers(1, &mut buffer.vbo);

            gl.BindVertexArray(buffer.vao);
            gl.BindBuffer(ARRAY_BUFFER, buffer.vbo);
            let stride: i32 =entries
                .iter()
                .map(|e| e.size)
                .sum::<i32>() * mem::size_of::<f32>() as i32;

            let mut cumulative_offset: i32 = 0;
            for entry in entries.iter() {
                gl.VertexAttribPointer(
                    entry.index,
                    entry.size,
                    FLOAT,
                    FALSE,
                    stride,
                    (cumulative_offset * mem::size_of::<f32>() as i32) as _,
                );
                gl.EnableVertexAttribArray(entry.index);
                cumulative_offset += entry.size;
            }
        }
        gl_error(gl);
        buffer
    }

    fn bind(&self, gl: &Gl) {
        gl_error(gl);
        unsafe {
            gl.BindVertexArray(self.vao);
            gl.BindBuffer(ARRAY_BUFFER, self.vbo);
        }
        gl_error(gl);
    }

    pub fn upload_buffers(&self, gl: &Gl) {
        gl_error(gl);
        if self.vertex_buffer.is_empty() { return; }
        unsafe {
            self.bind(gl);
            gl.BufferData(
                ARRAY_BUFFER,
                (self.vertex_buffer.len() * std::mem::size_of::<f32>()) as isize,
                &self.vertex_buffer[0] as *const f32 as *const c_void,
                STATIC_DRAW,
            );
            gl.BindVertexArray(0);
        }
        gl_error(gl);
    }

    pub fn draw_elements(&self, gl: &Gl, shader: &Shader, texture: &Texture) {
        gl_error(gl);
        unsafe {
            self.bind(gl);
            shader.activate(gl);
            texture.bind_texture(gl);
            gl.Enable(BLEND);
            gl.BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
            gl.DrawArrays(TRIANGLES, 0, self.vertex_buffer.len() as i32 / self.element_size as i32);
            gl.Disable(BLEND);
            gl.BindVertexArray(0);
        }
        gl_error(gl);
    }

    pub fn draw_elements_no_texture(&self, gl: &Gl, shader: &Shader) {
        gl_error(gl);
        unsafe {
            self.bind(gl);
            shader.activate(gl);
            gl.Enable(BLEND);
            gl.BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
            gl.DrawArrays(TRIANGLES, 0, self.vertex_buffer.len() as i32 / self.element_size as i32);
            gl.Disable(BLEND);
            gl.BindVertexArray(0);
        }
        gl_error(gl);
    }

    pub fn add3(&mut self, a: f32, b:f32, c:f32) {
        self.vertex_buffer.push(a);
        self.vertex_buffer.push(b);
        self.vertex_buffer.push(c);
    }

    pub fn add4(&mut self, a: f32, b:f32, c:f32, d:f32) {
        self.vertex_buffer.push(a);
        self.vertex_buffer.push(b);
        self.vertex_buffer.push(c);
        self.vertex_buffer.push(d);
    }

    pub fn add_slice(&mut self, s: &[f32]) {
        self.vertex_buffer.extend_from_slice(s);
    }
}