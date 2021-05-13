
use super::*;

use image::io::Reader as ImageReader;

pub struct Texture
{
    m_id: IdType,
    m_index: GLenum
}

impl Texture
{
    pub fn new(shader: &Shader, file_path: &str, texture_name: &str, index: GLenum, gen_mipmap: bool) -> Self
    {
        let mut texture = Texture{ m_id: 0, m_index: index };

        let image = ImageReader::open(file_path).expect("cannot open image file")
            .decode().unwrap().flipv().into_rgba8();
        
        gl_call!(gl::GenTextures(1, &mut texture.m_id));

        texture.bind();

        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S    , gl::REPEAT as GLint));
        gl_call!(gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T    , gl::REPEAT as GLint));
        

        gl_call!(gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA8 as GLint, 
            image.width() as GLsizei, image.height() as GLsizei, 0, 
            gl::RGBA, gl::UNSIGNED_BYTE, image.as_ptr() as VoidPtr));
        
        if gen_mipmap
        {
            gl_call!(gl::GenerateMipmap(gl::TEXTURE_2D));
        }

        gl_call!(gl::Uniform1i(shader.get_uniform(texture_name), index as i32));

        texture.unbind();

        texture
    }

    pub fn bind(&self)
    {
        gl_call!(gl::ActiveTexture(gl::TEXTURE0 + self.m_index));
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, self.m_id));
    }

    pub fn unbind(&self)
    {
        gl_call!(gl::BindTexture(gl::TEXTURE_2D, 0));
    }

}


impl Drop for Texture
{
    fn drop(&mut self)
    {
        gl_call!(gl::DeleteTextures(1, &self.m_id));
    }
}
