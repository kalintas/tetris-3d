
use std::time::Instant;

use rand::{Rng, SeedableRng, rngs::StdRng, thread_rng};

use nalgebra_glm as glm;

type Vec2 = glm::TVec2::<i32>;
type Mat2 = glm::TMat2::<i32>;
type SeedT = u64;

const EMPTY_BLOCK: SeedT = std::u64::MAX;
const O_TETROMINO: usize = 3; // O-block index
const BLOCKS_MOVE_SPEED: f32 = 0.05;

const TETROMINO_COORDS: [[Vec2; 3]; 7] = 
[
    [ Vec2::new( -1,  0 ), Vec2::new(  1, 0 ), Vec2::new( 2,  0 ) ], // I-tetromino
    [ Vec2::new( -1, -1 ), Vec2::new( -1, 0 ), Vec2::new( 1,  0 ) ], // J-tetromino
    [ Vec2::new( -1,  0 ), Vec2::new(  1, 0 ), Vec2::new( 1, -1 ) ], // L-tetromino
    [ Vec2::new(  1,  0 ), Vec2::new(  0, 1 ), Vec2::new( 1,  1 ) ], // O-tetromino
    [ Vec2::new( -1,  1 ), Vec2::new(  0, 1 ), Vec2::new( 1,  0 ) ], // S-tetromino
    [ Vec2::new( -1,  0 ), Vec2::new(  0, 1 ), Vec2::new( 1,  0 ) ], // T-tetromino
    [ Vec2::new( -1,  0 ), Vec2::new(  0, 1 ), Vec2::new( 1,  1 ) ], // Z-tetromino
];

struct DroppingPiece
{
    pub pos: Vec2,
    pub drop_pos: i32,
    pub piece_type: usize,
    pub rotation_mat: Mat2,

    pub draw_pos: glm::Vec2,
}

impl DroppingPiece
{
    fn new(pos: Vec2) -> Self
    {
        DroppingPiece
        { 
            pos, drop_pos: 0, draw_pos: pos.cast::<f32>(),
            piece_type: thread_rng().gen_range(0..TETROMINO_COORDS.len()),
            rotation_mat: glm::identity::<i32, 2>()
        }
    }
    
    fn into_new(&mut self, pos_y: i32)
    {
        self.pos.y = pos_y;
        self.draw_pos.y = pos_y as f32;

        self.drop_pos = 0;

        self.piece_type = thread_rng().gen_range(0..TETROMINO_COORDS.len());
        self.rotation_mat = glm::identity::<i32, 2>();
    }

    fn get_coord_at(&self, index: usize) -> Vec2
    {
        self.rotation_mat * TETROMINO_COORDS[self.piece_type][index]
    }

    fn get_pos_at(&self, index: usize) -> Vec2
    {   
        if index > 0
        {
            self.pos + self.get_coord_at(index - 1)
        }
        else 
        {
            self.pos
        }
    }

    fn draw_with(&self, func: impl Fn(glm::Vec2) -> ())
    {
        let vec = glm::vec2(self.pos.x as f32 - self.draw_pos.x, self.draw_pos.y);

        func(vec);

        for i in 0..3
        {
            func(vec + self.get_coord_at(i).cast::<f32>());
        }
    }

    fn draw_dropped_with(&self, func: impl Fn(glm::Vec2) -> ())
    {
        let vec = glm::vec2(self.pos.x as f32 - self.draw_pos.x, self.drop_pos as f32);
        
        func(vec);

        for i in 0..3
        {
            func(vec + self.get_coord_at(i).cast::<f32>());
        }
    }

    fn rotate(&mut self)
    {
        let (i_x, i_y) = (-self.rotation_mat[0], -self.rotation_mat[2]);
        let (j_x, j_y) = ( self.rotation_mat[1],  self.rotation_mat[3]);

        self.rotation_mat = glm::mat2(j_x, j_y, i_x, i_y);
    }
}


fn generate_color(seed: SeedT, alpha: f32) -> Option<glm::Vec4>
{   
    match seed
    {
        EMPTY_BLOCK => None,
        _ => 
        {
            let mix_color: glm::Vec3 = glm::vec3(0.4, 0.2, 0.8);
    
            let mut rng = StdRng::seed_from_u64(seed);
            
            let color = (glm::vec3(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>()) + mix_color) / 2.0;

            Some(glm::vec4(color.x, color.y, color.z, alpha)) 
        }
    }
}

pub struct GameLogic
{
    pub grid_width: usize,
    pub grid_height: usize,

    grid: Vec<SeedT>,

    drop_timer: Instant,
    drop_time: u128,

    current_seed: SeedT,
    current_piece: DroppingPiece,
}

impl GameLogic
{
    pub fn new(grid_width: usize, grid_height: usize) -> Self
    {   
        let mut grid = Vec::<SeedT>::new();

        grid.resize(grid_width * grid_height, EMPTY_BLOCK);
        
        let mut game = GameLogic
        {
            grid_width,
            grid_height,
            
            grid,            

            current_piece: DroppingPiece::new(glm::vec2(grid_width as i32 / 2, -2)),

            drop_timer: Instant::now(),
            drop_time: 500,
            
            current_seed: 0,
        };

        game.update_dropped_pos();

        game
    }

    pub fn update(&mut self)
    {   
        let diff = self.current_piece.pos.x as f32 - self.current_piece.draw_pos.x;

        self.current_piece.draw_pos.x += diff * BLOCKS_MOVE_SPEED;

        let elapsed = self.drop_timer.elapsed().as_millis();

        self.current_piece.draw_pos.y = self.current_piece.pos.y as f32 + elapsed as f32 / self.drop_time as f32;

        if elapsed >= self.drop_time
        {
            self.current_piece.pos.y += 1;

            if self.current_piece.pos.y >= self.current_piece.drop_pos
            {
                self.create_new_piece();
            }
            
            self.drop_timer = Instant::now();
        }
    }

    pub fn draw_grid_with(&self, func: impl Fn(f32, f32, Option<glm::Vec4>) -> ())
    {
        let draw_x = self.current_piece.draw_pos.x;

        for (i, it) in self.grid.iter().enumerate()
        {
            let x = (i % self.grid_width) as f32 - draw_x;

            func(x, (i / self.grid_width) as f32, generate_color(*it, 1.0));
        }

        self.current_piece        .draw_with(|v| func(v.x, v.y, generate_color(self.current_seed, 1.0)) );
        self.current_piece.draw_dropped_with(|v| func(v.x, v.y, generate_color(self.current_seed, 0.5)) );
    }

    pub fn move_piece(&mut self, movement: i32)
    {
        self.current_piece.pos.x += movement;

        if self.is_piece_collided()
        {
            self.current_piece.pos.x -= movement;
        }
        else
        {
            self.update_dropped_pos();
        }
    }

    pub fn toggle_piece_drop(&mut self, start_drop: bool)
    {
        if start_drop { self.drop_time /= 4; }
        else          { self.drop_time *= 4; }
    }

    pub fn hard_drop_piece(&mut self)
    {   
        self.current_piece.pos.y = self.current_piece.drop_pos;
        self.create_new_piece();
    }

    pub fn rotate_piece(&mut self)
    {
        if self.current_piece.piece_type == O_TETROMINO 
        {
            // O block doesnt need any rotation
            return;
        }

        let mat = self.current_piece.rotation_mat;

        self.current_piece.rotate();

        if self.is_piece_collided()
        {
            self.current_piece.rotation_mat = mat;
        }
        else
        {
            self.update_dropped_pos();
        }
    }
    
    fn create_new_piece(&mut self)
    {
        self.place_piece_to_grid();
        self.clear_lines();
        self.current_piece.into_new(2);
        self.update_dropped_pos();
    }

    fn update_dropped_pos(&mut self)
    {
        let old_pos = self.current_piece.pos.y;

        for y in old_pos + 1..self.grid_height as i32 + 1
        {
            self.current_piece.pos.y = y;

            if self.is_piece_collided()
            {
                self.current_piece.drop_pos = self.current_piece.pos.y - 1;
                break;
            }
        }

        self.current_piece.pos.y = old_pos;
    }

    fn is_piece_collided(&self) -> bool
    {
        if self.current_piece.pos.y < 1 { return false; }

        for i in 0..4
        {
            if self.at_grid(self.current_piece.get_pos_at(i)) != Some(EMPTY_BLOCK)
            {
                return true;
            }
        }
        
        false
    }

    fn at_grid(&self, v: Vec2) -> Option<SeedT>
    {
        if v.y >= 0 && v.y < self.grid_height as i32
        {   
            let x = (v.x as i32).rem_euclid(self.grid_width as i32) as usize;

            Some(self.grid[(v.y as usize * self.grid_width) + x])
        }
        else
        {
            None
        }
    }

    fn set_grid(&mut self, v: Vec2, value: SeedT)
    {   
        if v.y >= 0
        {
            let x = (v.x as i32).rem_euclid(self.grid_width as i32) as usize;
    
            self.grid[(v.y as usize * self.grid_width) + x] = value;
        }
    }

    fn place_piece_to_grid(&mut self)
    {   
        for i in 0..4
        {
            self.set_grid(self.current_piece.get_pos_at(i), self.current_seed);
        }

        self.current_seed += 1;
    }

    fn is_layer_full(&self, y: usize) -> bool
    {
        let index = y * self.grid_width;

        for i in 0..self.grid_width
        {
            if self.grid[index + i] == EMPTY_BLOCK { return false; }
        }

        true
    }

    fn should_clear_lines(&self) -> bool
    {
        for i in 0..4
        {
            if self.is_layer_full(self.current_piece.get_pos_at(i).y as usize)
            {
                return true;
            }
        }

        return false;
    }

    fn clear_lines(&mut self)
    {
        if !self.should_clear_lines() { return; } 

        for i in (1..self.grid_height).rev()
        {
            for t in 0..self.grid_width
            {
                self.grid[(i * self.grid_width) + t] = self.grid[((i - 1) * self.grid_width) + t];
            }

        }
    }

}

