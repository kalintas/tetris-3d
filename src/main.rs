
mod utils;
mod tetris;
fn main() 
{
    tetris::Tetris::new(640, 640, "3d-tetris").run();
}
