#![cfg_attr(not(test), no_std)]

use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, is_drawable};
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
##################                                                             #
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
}

impl Game {
    pub fn new() -> Self {
        Self {player: Player::new(), walls: Walls::new(WALLS)}
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

    pub fn tick(&mut self) {
        self.walls.draw();
        plot('*', self.player.x, self.player.y, ColorCode::new(Color::Green, Color::Black));
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
                plot(self.char_at(row, col), col, row, ColorCode::new(Color::Blue, Color::Black));
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
}

impl Player {
    pub fn new() -> Self {
        Self {x: BUFFER_WIDTH / 2, y: BUFFER_HEIGHT / 2}
    }

    pub fn is_colliding(&self, walls: &Walls) -> bool {
        walls.occupied(self.y, self.x)
    }

    pub fn down(&mut self) {
        self.y += 1;
    }

    pub fn up(&mut self) {
        self.y -= 1;
    }

    pub fn left(&mut self) {
        self.x -= 1;
    }

    pub fn right(&mut self) {
        self.x += 1;
    }
}
