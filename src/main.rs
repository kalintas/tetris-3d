mod tetris;
mod utils;
fn main() {
    tetris::Tetris::new(640, 640, "3d-tetris").run();
}
