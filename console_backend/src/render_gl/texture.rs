use crate::resources::Resources;
use image::{DynamicImage, GenericImageView};
use crate::render_gl::errors::Error;
use gl::Gl;
use failure::_core::ffi::c_void;

pub struct Texture {
    texture:  gl::types::GLuint,
    gl: Gl,
}

impl Texture {
    #[allow(dead_code)]
    pub fn from_res(gl: &Gl, res: &Resources, name: &str, rgb_type: gl::types::GLenum) -> Result<Self, Error> {
        let img = res.load_image(name).map_err(|err| {
            Error::ResourceLoad {
                inner: err,
                name: name.into(),
            }
        })?;
        Texture::from_img(gl, img, rgb_type)
    }

    pub fn from_img(gl: &Gl, img: DynamicImage, rgb_type: gl::types::GLenum) -> Result<Self, Error> {
        let img = img.flipv();
        let mut texture = 0;
        unsafe {
            gl.GenTextures(1, &mut texture);
            gl.BindTexture(gl::TEXTURE_2D, texture);

            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_BORDER as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_BORDER as i32);

            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }

        unsafe {
            gl.TexImage2D(gl::TEXTURE_2D,
                          0,
                          gl::RGBA as i32,
                          img.width() as i32,
                          img.height() as i32,
                          0,
                          rgb_type,
                          gl::UNSIGNED_BYTE,
                          &img.raw_pixels()[0] as *const u8 as *const c_void);
            gl.GenerateMipmap(gl::TEXTURE_2D);
        }
        Ok(Texture {
            texture,
            gl: gl.clone(),
        })
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindTexture(gl::TEXTURE_2D, self.texture);
        }
    }
}