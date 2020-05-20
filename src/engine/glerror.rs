use crate::opengl::*;

#[macro_export]
macro_rules! gl_error_wrap {
    ($gl:expr, $call:expr) => {
        $call;
        #[cfg(debug_assertions)]
        crate::hal::gl_error($gl);
    };
}

pub fn gl_error(gl: &Gl) {
    let error;
    unsafe {
        error = gl.GetError();
    }
    if error != NO_ERROR {
        match error {
            INVALID_ENUM => panic!("[GL] Error: INVALID_ENUM"),
            INVALID_VALUE => panic!("[GL] Error: INVALID_ENUM"),
            INVALID_OPERATION => panic!("[GL] Error: INVALID_ENUM"),
            STACK_OVERFLOW => panic!("[GL] Error: INVALID_ENUM"),
            STACK_UNDERFLOW => panic!("[GL] Error: INVALID_ENUM"),
            OUT_OF_MEMORY => panic!("[GL] Error: INVALID_ENUM"),
            INVALID_FRAMEBUFFER_OPERATION => panic!("[GL] Error: INVALID_ENUM"),
            _ => panic!("[GL] Error: {}", error),
        }
    }
}