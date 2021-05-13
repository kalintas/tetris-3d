
pub use gl::types::*;

pub use std::mem;
pub use std::ffi;

pub use gl::types;

pub type VoidPtr = *const ffi::c_void;
pub type IdType = GLuint;
pub type Uniform = GLint;

pub fn clear_gl_errors()
{
    while unsafe { gl::GetError() } != gl::NO_ERROR {}
}

pub fn check_gl_errors()
{
    loop
    {
        match unsafe { gl::GetError() }
        {
            gl::NO_ERROR => break,
            error => 
            { 
                println!("[OpenGL Error]: {}", error);
                panic!();
            }
        }
    }
    
}



#[macro_export]
macro_rules! gl_call 
{
    ($x: expr) => 
    {        
        crate::utils::clear_gl_errors();

        unsafe { $x; }
        
        crate::utils::check_gl_errors();
    };
}

pub mod shader;
pub use shader::Shader;

pub mod buffer_object;
pub use buffer_object::BufferObject;

pub mod vertex_array_object;
pub use vertex_array_object::VertexArrayObject;

pub mod texture;
pub use texture::Texture;


