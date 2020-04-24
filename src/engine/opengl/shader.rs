use super::support::gl::*;
use super::support::gl::types::*;
use std::ptr;
use std::str;

pub struct Shader {
    pub id: u32
}

impl Shader {
    pub fn new(gl: &Gl, vertex_code: &str, fragment_code: &str) -> Shader {
        // 1. compile shaders from strings
        let shader;
        let mut success = i32::from(FALSE);
        let mut info_log = Vec::with_capacity(512);
        unsafe {
            // vertex shader
            let vertex = gl.CreateShader(VERTEX_SHADER);
            gl.ShaderSource(vertex, 1, [vertex_code.as_ptr() as *const _].as_ptr(),std::ptr::null());
            gl.CompileShader(vertex);
            gl.GetShaderiv(vertex, COMPILE_STATUS, &mut success);
            if success != i32::from(TRUE) {
                gl.GetShaderInfoLog(
                    vertex,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                    str::from_utf8(&info_log).unwrap()
                );
                panic!();
            }

            // fragment Shader
            let fragment = gl.CreateShader(FRAGMENT_SHADER);
            gl.ShaderSource(fragment, 1, [fragment_code.as_ptr() as *const _].as_ptr(),std::ptr::null());
            gl.CompileShader(fragment);
            gl.GetShaderiv(fragment, COMPILE_STATUS, &mut success);
            if success != i32::from(TRUE) {
                gl.GetShaderInfoLog(
                    fragment,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
                    str::from_utf8(&info_log).unwrap()
                );
                panic!();
            }

            // shader Program
            let id = gl.CreateProgram();
            gl.AttachShader(id, vertex);
            gl.AttachShader(id, fragment);
            gl.LinkProgram(id);
            gl.GetProgramiv(id, LINK_STATUS, &mut success);
            if success != i32::from(TRUE) {
                gl.GetProgramInfoLog(
                    id,
                    512,
                    ptr::null_mut(),
                    info_log.as_mut_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                    str::from_utf8(&info_log).unwrap()
                );
                panic!();
            }
            gl.DeleteShader(vertex);
            gl.DeleteShader(fragment);

            shader = Shader { id }
        }

        shader
    }

    pub fn activate(&self, gl: &Gl) {
        unsafe {
            gl.UseProgram(self.id);
        }
    }
}