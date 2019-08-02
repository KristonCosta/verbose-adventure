
use gl::Gl;
use std::ffi::{CStr, CString};
use crate::resources::Resources;
use crate::render_gl::errors::Error;

pub struct Program {
    id: gl::types::GLuint,
    gl: Gl,
}

impl Program {
    pub fn from_res(gl: &Gl, res: &Resources, name: &str) -> Result<Self, Error> {
        const POSSIBLE_EXT: [&str; 2] = [
            ".vert",
            ".frag",
        ];

        let shaders = POSSIBLE_EXT.iter()
            .map(|file_extension| {
                Shader::from_res(gl, res, &format!("{}{}", name, file_extension))
            })
            .collect::<Result<Vec<Shader>, Error>>()?;

        Program::from_shaders(gl, &shaders).map_err(|message| Error::LinkError {
            name: name.into(),
            message,
        })
    }

    pub fn from_shaders(gl: &Gl, shaders: &[Shader]) -> Result<Self, String> {
        let program_id = unsafe {
            gl.CreateProgram()
        };

        for shader in shaders {
            unsafe {
                gl.AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl.LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl.GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar
                );
            }
            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl.DetachShader(program_id, shader.id());
            }
        }

        Ok(Program { id: program_id, gl: gl.clone() })
    }

    pub fn set_used(&self) {
        unsafe {
            self.gl.UseProgram(self.id);
        }
    }

    #[allow(dead_code)]
    pub fn set_bool(&self, name: &str, value: bool) {
        let location = self.get_uniform_location(name);
        unsafe { self.gl.Uniform1i(location, value as gl::types::GLint) }
    }

    #[allow(dead_code)]
    pub fn set_int(&self, name: &str, value: i32) {
        let location = self.get_uniform_location(name);
        unsafe { self.gl.Uniform1i(location, value as gl::types::GLint) }
    }

    #[allow(dead_code)]
    pub fn set_float(&self, name: &str, value: f32) {
        let location = self.get_uniform_location(name);
        unsafe { self.gl.Uniform1f(location, value as gl::types::GLfloat) }
    }

    #[allow(dead_code)]
    pub fn set_mat_4f(&self, name: &str, value: nalgebra_glm::Mat4) {
        let location = self.get_uniform_location(name);
        unsafe { self.gl.UniformMatrix4fv(location, 1, gl::FALSE, value.as_ptr() as *const gl::types::GLfloat)}
    }

    #[allow(dead_code)]
    pub fn get_uniform_location(&self, name: &str) -> i32 {
        let name = CString::new(name).unwrap();
        unsafe {
            self.gl.GetUniformLocation(self.id, name.as_ptr() as *const gl::types::GLchar)
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}

pub struct Shader {
    id: gl::types::GLuint,
    gl: Gl,
}

impl Shader {
    pub fn from_res(gl: &Gl, res: &Resources, name: &str) -> Result<Self, Error> {
        const POSSIBLE_EXT: [(&str, gl::types::GLenum); 2] = [
            (".vert", gl::VERTEX_SHADER),
            (".frag", gl::FRAGMENT_SHADER),
        ];

        let shader_kind = POSSIBLE_EXT.iter()
            .find(|&&(file_extension, _)| {
                name.ends_with(file_extension)
            })
            .map(|&(_, kind)| kind)
            .ok_or_else(|| Error::CanNotDetermineShaderTypeForResource { name: name.into() })?;
        let source = res.load_cstring(name)
            .map_err(|e| Error::ResourceLoad { name: name.into(), inner: e })?;
        Shader::from_source(gl, &source, shader_kind).map_err(|message| Error::CompileError {
            name: name.into(),
            message,
        })
    }

    pub fn from_source(gl: &Gl, source: &CStr, kind: gl::types::GLuint) -> Result<Self, String> {
        let id = Shader::shader_from_source(&gl, source, kind)?;
        Ok(Shader {
            id,
            gl: gl.clone(),
        })
    }

    #[allow(dead_code)]
    pub fn from_vert_source(gl: &Gl, source: &CStr) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::VERTEX_SHADER)
    }

    #[allow(dead_code)]
    pub fn from_frag_source(gl: &Gl, source: &CStr) -> Result<Shader, String> {
        Shader::from_source(gl, source, gl::FRAGMENT_SHADER)
    }

    fn shader_from_source(gl: &Gl, source: &CStr, kind: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
        let id = unsafe {
            gl.CreateShader(kind)
        };

        unsafe {
            gl.ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl.CompileShader(id);
        };

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }
            let error: CString = create_whitespace_cstring_with_len(len as usize);
            unsafe {
                gl.GetShaderInfoLog(id,
                                    len,
                                    std::ptr::null_mut(),
                                    error.as_ptr() as *mut gl::types::GLchar);
            }
            return Err(error.to_string_lossy().into_owned());
        }
        Ok(id)
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len as usize + 1);
    buffer.extend([b' '].iter().cycle().take(len as usize));
    unsafe { CString::from_vec_unchecked(buffer) }
}