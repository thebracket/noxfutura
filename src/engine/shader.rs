use super::support::gl::*;
use super::support::gl::types::*;
use super::GL;
use std::ptr;
use std::str;
use std::ffi::CString;

pub struct Shader(pub u32);

impl Shader {
    pub fn new_selflock(vertex_code: &str, fragment_code: &str) -> Shader {
        let glock = GL.lock();
        let gl = glock.gl.as_ref().unwrap();
        Self::new(gl, vertex_code, fragment_code)
    }

    pub fn new(gl: &Gl, vertex_code: &str, fragment_code: &str) -> Shader {
        let v_src = CString::new(vertex_code.as_bytes()).unwrap();
        let f_src = CString::new(fragment_code.as_bytes()).unwrap();

        // 1. compile shaders from strings
        let shader;
        let mut success = i32::from(FALSE);
        let info_log = vec![b' '; 512];
        let info_error : CString = unsafe { CString::from_vec_unchecked(info_log) };
        unsafe {
            // vertex shader
            let vertex = gl.CreateShader(VERTEX_SHADER);
            gl.ShaderSource(vertex, 1, [v_src.as_ptr() as *const _].as_ptr(), std::ptr::null());
            gl.CompileShader(vertex);
            gl.GetShaderiv(vertex, COMPILE_STATUS, &mut success);
            if success != i32::from(TRUE) {
                gl.GetShaderInfoLog(
                    vertex,
                    512,
                    ptr::null_mut(),
                    info_error.as_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::VERTEX::COMPILATION_FAILED\n{}",
                    info_error.to_string_lossy()
                );
                panic!();
            }

            // fragment Shader
            let fragment = gl.CreateShader(FRAGMENT_SHADER);
            gl.ShaderSource(fragment, 1, [f_src.as_ptr() as *const _].as_ptr(),std::ptr::null());
            gl.CompileShader(fragment);
            gl.GetShaderiv(fragment, COMPILE_STATUS, &mut success);
            if success != i32::from(TRUE) {
                gl.GetShaderInfoLog(
                    fragment,
                    512,
                    ptr::null_mut(),
                    info_error.as_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::FRAGMENT::COMPILATION_FAILED\n{}",
                    info_error.to_string_lossy()
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
                    info_error.as_ptr() as *mut GLchar,
                );
                println!(
                    "ERROR::SHADER::PROGRAM::COMPILATION_FAILED\n{}",
                    info_error.to_string_lossy()
                );
                panic!();
            }
            gl.DeleteShader(vertex);
            gl.DeleteShader(fragment);

            shader = Shader(id)
        }

        shader
    }

    pub fn activate_selflock(&self) {
        let glock = GL.lock();
        let gl = glock.gl.as_ref().unwrap();
        self.activate(gl);
    }

    pub fn activate(&self, gl: &Gl) {
        unsafe {
            gl.UseProgram(self.0);
        }
    }
}