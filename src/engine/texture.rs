#![allow(dead_code)]
use image::GenericImageView;
use crate::opengl::*;

pub struct Texture(u32);

impl Texture {
    pub fn from_file<S: ToString>(gl: &Gl, filename : S) -> Self {
        let image = image::open(std::path::Path::new(&filename.to_string()))
            .expect("Failed to load texture");

        let mut texture : u32 = 0;

        unsafe {
            gl.GenTextures(1, &mut texture);
            gl.BindTexture(TEXTURE_2D, texture);
            gl.TexParameteri(TEXTURE_2D, TEXTURE_WRAP_S, CLAMP_TO_EDGE as _);
            gl.TexParameteri(TEXTURE_2D, TEXTURE_WRAP_T, CLAMP_TO_EDGE as _);
            gl.TexParameteri(TEXTURE_2D, TEXTURE_MIN_FILTER, LINEAR as _);
            gl.TexParameteri(TEXTURE_2D, TEXTURE_MAG_FILTER, LINEAR as _);
            let w = image.width() as i32;
            let h = image.height() as i32;
            let data = image.into_rgba().into_raw();
            gl.TexImage2D(
                TEXTURE_2D,
                0,
                RGBA as _,
                w,
                h,
                0,
                RGBA,
                UNSIGNED_BYTE,
                data.as_ptr() as _
            );

            gl_error(gl);
        }

        Self(texture)
    }

    pub fn bind_texture(&self, gl: &Gl) {
        unsafe {
            gl.BindTexture(TEXTURE_2D, self.0);
            gl_error(gl);
        }
    }
}
