
use super::*;

pub struct VertexArrayObject
{
    m_id: IdType
}

impl VertexArrayObject
{
    pub fn new() -> Self
    {
        let mut obj = VertexArrayObject{ m_id: 0 };

        gl_call!(gl::GenVertexArrays(1, &mut obj.m_id));

        obj
    }

    pub fn bind(&self)
    {
        gl_call!(gl::BindVertexArray(self.m_id));
    }

    #[allow(dead_code)]
    pub fn unbind(&self)
    {
        gl_call!(gl::BindVertexArray(0));
    }
}


impl Drop for VertexArrayObject
{
    fn drop(&mut self)
    {
        gl_call!(gl::DeleteVertexArrays(1, &self.m_id));
    }
}

