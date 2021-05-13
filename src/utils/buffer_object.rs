
use crate::{utils::*, gl_call};



pub struct BufferObject
{
    m_id: IdType,
    m_target: GLenum
}

impl BufferObject
{
    pub fn new<T>(target: GLenum, buffer: &[T], usage: GLenum) -> Self
    {   
        let mut obj = BufferObject{ m_id: 0, m_target: target };

        gl_call!(gl::GenBuffers(1, &mut obj.m_id));
        
        obj.bind();
        gl_call!(gl::BufferData(obj.m_target, (buffer.len() * mem::size_of::<T>()) as GLsizeiptr,
            buffer.as_ptr() as VoidPtr, usage));

        obj.unbind();
    
        obj
    }

    pub fn bind(&self)
    {
        gl_call!(gl::BindBuffer(self.m_target, self.m_id));
    }

    pub fn unbind(&self)
    {
        gl_call!(gl::BindBuffer(self.m_target, 0));
    }
    
    pub fn create_vertex(index: GLuint, size: GLint, vertex_type: GLenum, 
            normalized: GLboolean, stride: usize, start_point: usize)
    {
        gl_call!(gl::VertexAttribPointer(index, size, vertex_type, normalized, stride as GLsizei, start_point as VoidPtr));
        gl_call!(gl::EnableVertexAttribArray(index));
    }   

}

impl Drop for BufferObject
{
    fn drop(&mut self)
    {
        gl_call!(gl::DeleteBuffers(1, &mut self.m_id));
    }
}