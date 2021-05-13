
use crate::{utils::*, gl_call};

use std::fs;
use ffi::CString;

pub struct Shader
{
    m_id: GLuint
}

impl Shader
{
    pub fn new(vertex_fp: &str, fragment_fp: &str) -> Self
    {
        let mut shader = Shader{ m_id: 0 };

        gl_call!(shader.m_id = gl::CreateProgram());

        shader.create_shader(&vertex_fp, gl::VERTEX_SHADER);
        shader.create_shader(&fragment_fp, gl::FRAGMENT_SHADER);
        
        gl_call!(gl::LinkProgram(shader.m_id));
        gl_call!(gl::ValidateProgram(shader.m_id));
        
        shader
    }

    fn compile_shader(shader_type: GLenum, shader_str: &str) -> Result<GLuint, String>
    {
        let shader_id;
                
        gl_call!(shader_id = gl::CreateShader(shader_type));

        let shader_src = CString::new(shader_str).unwrap();

        gl_call!(gl::ShaderSource(shader_id, 1,  &shader_src.as_ptr(), std::ptr::null()));
        gl_call!(gl::CompileShader(shader_id));
        
        let mut success: GLint = 0;
        
        gl_call!(gl::GetShaderiv(shader_id, gl::COMPILE_STATUS, &mut success));

        if success == gl::TRUE as GLint { return Ok(shader_id); }

        let mut error_length: GLint = 0;
        
        gl_call!(gl::GetShaderiv(shader_id, gl::INFO_LOG_LENGTH, &mut error_length));
        
        let mut error_string = Vec::<GLchar>::new();

        error_string.resize(error_length as usize, 0i8);

        gl_call!(gl::GetShaderInfoLog(shader_id, error_length, &mut error_length, error_string.as_mut_ptr()));

        unsafe
        {
            Err(String::from_utf8(mem::transmute(error_string)).unwrap())
        }
    }

    fn create_shader(&self, file_path: &str, shader_type: GLenum)
    {
        let shader_str = fs::read_to_string(file_path).unwrap();

        let shader_id = Self::compile_shader(shader_type, &shader_str).unwrap();

        gl_call!(gl::AttachShader(self.m_id, shader_id));
        gl_call!(gl::DeleteShader(shader_id));
    }

    pub fn bind(&self)
    {
        gl_call!(gl::UseProgram(self.m_id));
    }

    #[allow(dead_code)]
    pub fn unbind(&self)
    {
        gl_call!(gl::UseProgram(0));
    }

    pub fn get_uniform(&self, uniform_name: &str) -> Uniform
    {
        let result: GLint;

        let c_str = CString::new(uniform_name).unwrap();

        gl_call!(result = gl::GetUniformLocation(self.m_id, c_str.as_ptr()));

        if result < 0
        {
            println!("[WARNING]: cannot find {} uniform", uniform_name);
        }

        result
    }
}

impl Drop for Shader
{
    fn drop(&mut self)
    {
        gl_call!(gl::DeleteProgram(self.m_id));
    }
}