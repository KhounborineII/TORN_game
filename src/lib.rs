#![cfg_attr(not(test), no_std)]

use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color};
use pc_keyboard::{DecodedKey, KeyCode};

const WALLS: &str = "################################################################################
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
################################################################################";

pub struct Game {
    player: Player,
    walls: Walls,
    light_path: Light
}

impl Game {
    pub fn new() -> Self {
        Self {player: Player::new(), walls: Walls::new(WALLS), light_path: Light::new()}
    }

    pub fn tick(&mut self) {
        self.walls.draw();
        self.bike_dir();
        self.moving();
        self.trail_blaze();
    }

    pub fn key(&mut self, key: DecodedKey) {
        match key {
            DecodedKey::RawKey(key) => {
                let mut future = self.player;
                match key {
                    KeyCode::ArrowDown => {
                        future.down();
                    }
                    KeyCode::ArrowUp => {
                        future.up();
                    } 
                    KeyCode::ArrowLeft => {
                        future.left();
                    }
                    KeyCode::ArrowRight => {
                        future.right();
                    }
                    _ => {}
                }
                if !future.is_colliding(&self.walls) {
                    self.player = future;
                }
            }
            DecodedKey::Unicode(_) => {}
        }
    }

    pub fn moving(&mut self){
        let mut future = self.player;
        if future.direction == 0 {
            future.x += 1;
        }
        else if future.direction == 1 {
            future.x -= 1;
        }
        else if future.direction == 2 {
            future.y += 1;
        }
        else if future.direction == 3 {
            future.y -= 1;
        }
        if !future.is_colliding(&self.walls){
            self.player = future;
        }
    }

    pub fn bike_dir(&mut self){
        if self.player.direction == 0 {
            plot('>', self.player.x, self.player.y, ColorCode::new(Color::LightBlue, Color::Black));
        }
        else if self.player.direction == 1 {
            plot('<', self.player.x, self.player.y, ColorCode::new(Color::LightBlue, Color::Black));
        }
        else if self.player.direction == 2 {
            plot('v', self.player.x, self.player.y, ColorCode::new(Color::LightBlue, Color::Black));
        }
        else if self.player.direction == 3 {
            plot('^', self.player.x, self.player.y, ColorCode::new(Color::LightBlue, Color::Black));
        }
    }

    pub fn trail_blaze(&mut self){
        self.light_path.user = self.player;
        self.light_path.setup();
        if self.light_path.axis == 0 {
            if self.player.direction == 0 {
                plot('-', self.player.x - 1, self.player.y, ColorCode::new(Color::LightBlue, Color::Black));
            }
            else if self.player.direction == 1 {
                plot('-', self.player.x + 1, self.player.y, ColorCode::new(Color::LightBlue, Color::Black));
            }
        }
        else if self.light_path.axis == 1 {
            if self.player.direction == 2 {
                plot('|', self.player.x, self.player.y - 1, ColorCode::new(Color::LightBlue, Color::Black));
            }
            else if self.player.direction == 3 {
                plot('|', self.player.x, self.player.y + 1, ColorCode::new(Color::LightBlue, Color::Black));
            }
        }
    }
}

pub struct Walls {
    walls: [[bool; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

impl Walls {
    pub fn new(map: &str) -> Self {
        let mut walls = [[false; BUFFER_WIDTH]; BUFFER_HEIGHT];
        for (row, chars) in map.split('\n').enumerate() {
            for (col, value) in chars.char_indices() {
                walls[row][col] = value == '#';
            }
        }
        Self {walls}
    }

    pub fn draw(&self) {
        for row in 0..self.walls.len() {
            for col in 0..self.walls[row].len() {
                plot(self.char_at(row, col), col, row, ColorCode::new(Color::White, Color::Black));
            }
        }
    }

    pub fn occupied(&self, row: usize, col: usize) -> bool {
        self.walls[row][col]
    }

    fn char_at(&self, row: usize, col: usize) -> char {
        if self.walls[row][col] {
            '#'
        } else {
            ' '
        }
    }
}

#[derive(Copy, Clone)]
pub struct Player {
    x: usize,
    y: usize,
    direction: usize
}

impl Player {
    pub fn new() -> Self {
        Self {x: BUFFER_WIDTH / 2, 
            y: BUFFER_HEIGHT / 2, 
            direction: 0
        }
    }

    pub fn is_colliding(&self, walls: &Walls) -> bool {
        walls.occupied(self.y, self.x)
    }

    pub fn down(&mut self) {
        if self.direction != 3 {
            self.direction =2;
        self.y += 1;
        }
    }

    pub fn up(&mut self) {
        if self.direction != 2 {
            self.direction = 3;
            self.y -= 1;
        }
    }

    pub fn left(&mut self) {
        if self.direction != 0 {
            self.direction = 1;
            self.x -= 1;
        }
    }

    pub fn right(&mut self) {
        if self.direction != 1 {
            self.direction = 0;
            self.x += 1;
        }
    }
}

#[derive(Copy, Clone)]
pub struct Light {
    x: usize,
    y: usize,
    axis: usize,
    user: Player
}

impl Light {
    pub fn new() -> Self {
        Self {x: BUFFER_WIDTH / 2, 
            y: BUFFER_HEIGHT / 2, 
            axis: 0,
            user: Player { x: 0, y: 0, direction: 0 }
        }
    }

    pub fn setup(&mut self){
        self.x = self.user.x;
        self.y = self.user.y;
        if self.user.direction == 2 || self.user.direction == 3 {
            self.axis = 1;
        }
        else if self.user.direction == 0 || self.user.direction == 1 {
            self.axis = 0;
        }
    }

}