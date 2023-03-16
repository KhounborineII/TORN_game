#![cfg_attr(not(test), no_std)]

use pluggable_interrupt_os::vga_buffer::{BUFFER_WIDTH, BUFFER_HEIGHT, plot, ColorCode, Color, plot_num};
use pc_keyboard::{DecodedKey, KeyCode};

const WALLS: &str = "


################################################################################
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

const GAMEOVER: &str = "


################################################################################
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                 DEREZZED                                     #
#                                                                              #
#                                 SCORE:                                       #
#                                                                              #
#                           PRESS 'R' TO RESTART                               #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
################################################################################";

const START: &str = "################################################################################
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                                                              #
#                                  TORN                                        #
#                                                                              #
#                                                                              #
#                            PRESS 'P' TO PLAY                                 #
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


const PLAYER: Player = Player {x: 0, y:0, direction: 0, controlled: 0, turn_check: 0};
const LIGHT: Light = Light { x: 0, y: 0, axis: 0, user: PLAYER};
const TRAIL: [Light; 25] = [LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, 
LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, LIGHT, ];

pub struct Game {
    player: Player,
    walls: Walls,
    light_path: LightPath,
    light: Light,
    game_over: GameOver,
    end_set: bool,
    points: usize,
    enemy: Player,
    dark_path: LightPath,
    dark: Light,
    game_fix: bool,
    start_screen: GameOver,
    game_start: bool,
    tick: isize
}

impl Game {
    pub fn new() -> Self {
        Self {player: Player::new(), 
            walls: Walls::new(WALLS), 
            light_path: LightPath::new(), 
            light: Light::new(),
            game_over: GameOver::new(GAMEOVER),
            end_set: false,
            points: 0,
            enemy: Player {x: (BUFFER_WIDTH *5) / 6, y: BUFFER_HEIGHT / 2, direction: 1, controlled: 0, turn_check: 10},
            dark_path: LightPath::new(),
            dark: Light::new(),
            game_fix: false,
            start_screen: GameOver::new(START),
            game_start: false,
            tick: 0
        }
    }

    pub fn tick(&mut self) {
        if !self.game_start{
            self.start_screen.over = false;
            self.start_screen.draw();
        }
        else{
            if !self.end_set {
                self.tick += 1;
                self.walls.draw();
                self.draw_title();
                self.player.bike_dir();
                self.enemy.motor_dir();
                self.trail_blaze();
                self.moving();
                self.groving();
                self.enemy.travel_stance(self.tick);
            }
            else {
                self.game_over.points = self.points;
                self.game_over.draw();
                self.draw_title();
            }
        }
        
    }

    pub fn draw_title(&mut self) {
        let title = "TORN";
        let score = "Score: ";
        let mut bop = 30;
        let mut pob = 45; 
        for c in title.chars() {
            plot(c, bop, 1, ColorCode::new(Color::LightBlue, Color::Black));
            bop += 1;
        }
        for c in score.chars() {
            plot(c, pob, 1, ColorCode::new(Color::LightBlue, Color::Black));
            pob += 1;
        }
        plot_num(self.points.try_into().unwrap(), 52, 1, ColorCode::new(Color::LightBlue, Color::Black));
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
                if !future.is_colliding(&self.walls) && !future.is_crashing(self.light_path) && !future.is_crashing(self.dark_path) {
                    self.player = future;
                }
            }
            DecodedKey::Unicode(key) => {
                let mut future = self.player;
                if key == 'r' {
                    self.end_set = false;
                    self.game_start = false;
                    future = Player::new();
                    self.game_fix = true;
                    self.player.reset();
                    self.enemy = Player {x: (BUFFER_WIDTH *5) / 6, y: BUFFER_HEIGHT / 2, direction: 1, controlled: 0, turn_check: 13};
                    self.dark_path = LightPath::new();
                    self.light_path = LightPath::new();
                }
                if key == 'e'{
                    self.end_set = true;
                }
                if key == 'p'{
                    self.game_start = true;
                }        
            }
            
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
        if self.game_fix{
            future.reset();
            self.redo();
        }
        else {
            if !future.is_colliding(&self.walls) && !future.is_crashing(self.light_path) && !future.is_crashing(self.dark_path){
                self.player = future;
            }
            else {
                self.end_set = true;
            }
        }
    }

    pub fn groving(&mut self) {
        let mut future = self.enemy;
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
        if !future.is_colliding(&self.walls) && !future.is_crashing(self.light_path) && !future.is_crashing(self.dark_path){
            self.enemy = future;
        }
        else if future.is_colliding(&self.walls) {
            if self.enemy.direction == 0 || self.enemy.direction == 1{
                self.enemy.down();
            }
            else if self.enemy.direction == 2 || self.enemy.direction == 3{
                self.enemy.left();
            }
        }
        else {
            self.points += 1;
            self.enemy = Player {x: (BUFFER_WIDTH *5) / 6, y: BUFFER_HEIGHT / 2, direction: 1, controlled: 0, turn_check: 13};
        }
    }

    pub fn trail_blaze(&mut self){
        self.light.user = self.player;
        self.light.setup();
        self.light_path.generate(self.light);
        self.light_path.trail_iter += 1;
        self.light_path.draw();
        self.dark.user = self.enemy;
        self.dark.setup();
        self.dark_path.generate(self.dark);
        self.dark_path.trail_iter += 1;
        self.dark_path.draw();
    }

    pub fn redo(&mut self) {
        self.end_set = false;
        self.player.reset();
        self.enemy = Player {x: (BUFFER_WIDTH *5) / 6, y: BUFFER_HEIGHT / 2, direction: 1, controlled: 0, turn_check: 13};
        self.game_fix = false;
        self.points = 0;
        self.tick = 0;
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
    direction: usize,
    controlled: usize,
    turn_check: isize
}

impl Player {
    pub fn new() -> Self {
        Self {x: (BUFFER_WIDTH / 6), 
            y: BUFFER_HEIGHT / 2, 
            direction: 0,
            controlled: 1,
            turn_check: 0
        }
    }

    pub fn reset(&mut self) {
        self.x = BUFFER_WIDTH / 6; 
        self.y = BUFFER_HEIGHT / 2; 
        self.direction = 0;
    }

    pub fn is_colliding(&self, walls: &Walls) -> bool {
        walls.occupied(self.y, self.x)
    }

    pub fn is_crashing(&self, trail: LightPath) -> bool {
        let mut i = 0;
        while i < trail.check_clear{
            if self.x == trail.trail[i].x && self.y == trail.trail[i].y {
                return true;
             }
            i += 1;
        }
        return false;   
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

    pub fn bike_dir(&mut self){
        if self.direction == 0 {
            plot('>', self.x, self.y, ColorCode::new(Color::LightBlue, Color::Black));
        }
        else if self.direction == 1 {
            plot('<', self.x, self.y, ColorCode::new(Color::LightBlue, Color::Black));
        }
        else if self.direction == 2 {
            plot('v', self.x, self.y, ColorCode::new(Color::LightBlue, Color::Black));
        }
        else if self.direction == 3 {
            plot('^', self.x, self.y, ColorCode::new(Color::LightBlue, Color::Black));
        }
    }

    pub fn motor_dir(&mut self){
        if self.direction == 0 {
            plot('>', self.x, self.y, ColorCode::new(Color::LightRed, Color::Black));
        }
        else if self.direction == 1 {
            plot('<', self.x, self.y, ColorCode::new(Color::LightRed, Color::Black));
        }
        else if self.direction == 2 {
            plot('v', self.x, self.y, ColorCode::new(Color::LightRed, Color::Black));
        }
        else if self.direction == 3 {
            plot('^', self.x, self.y, ColorCode::new(Color::LightRed, Color::Black));
        }
    }

    pub fn travel_stance(&mut self, tick: isize) {
        if self.turn_check > 0{
            self.turn_check -= 1;
        }
        else if self.turn_check == 0{
            if self.direction == 0  || self.direction == 1{
                if tick % 3 == 0 {
                    self.up();
                }
                else if tick % 2 == 0{
                    self.down();
                }
            }
            else if self.direction == 2 || self.direction == 3 {
                if tick % 3 == 0 {
                    self.left();
                }
                else if tick % 2 == 0{
                    self.right();
                }
            }
            self.turn_check = 4;
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
        Self {x: 0, 
            y: 0, 
            axis: 0,
            user: PLAYER
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

    pub fn draw(&mut self){
        if self.user.controlled == 1{
            if self.axis == 0 {
                self.y = self.user.y;
                if self.user.direction == 0 {self.x = self.user.x - 1;}//RIGHT
                else if self.user.direction == 1 {self.x = self.user.x + 1;} //LEFT
                plot('-', self.x, self.user.y, ColorCode::new(Color::LightBlue, Color::Black));
            }
            else if self.axis == 1 {
                self.x = self.user.x;
                if self.user.direction == 2 {self.y = self.user.y - 1;}//UP
                else if self.user.direction == 3 {self.y = self.user.y + 1;}//DOWN
                plot('|', self.user.x, self.y, ColorCode::new(Color::LightBlue, Color::Black));
            }
        }
        else if self.user.controlled == 0 {
            if self.axis == 0 {
                self.y = self.user.y;
                if self.user.direction == 0 {self.x = self.user.x - 1;}//RIGHT
                else if self.user.direction == 1 {self.x = self.user.x + 1;} //LEFT
                plot('-', self.x, self.user.y, ColorCode::new(Color::LightRed, Color::Black));
            }
            else if self.axis == 1 {
                self.x = self.user.x;
                if self.user.direction == 2 {self.y = self.user.y - 1;}//UP
                else if self.user.direction == 3 {self.y = self.user.y + 1;}//DOWN
                plot('|', self.user.x, self.y, ColorCode::new(Color::LightRed, Color::Black));
            }
        }    
    }
    

}

#[derive(Copy, Clone)]
pub struct LightPath {
    trail: [Light; 25],
    trail_iter: usize,
    check_clear: usize
}

impl LightPath {
    pub fn new() -> Self {
        Self {trail: TRAIL,
            trail_iter: 0,
            check_clear: 0
        }
    }

    pub fn generate(&mut self, light: Light) {
        if self.trail_iter >= self.trail.len(){
            self.trail_iter = 0;
        }
        self.trail[self.trail_iter] = light;
        if self.check_clear < self.trail.len() {
            self.check_clear += 1;
        }
    }

    pub fn draw(&mut self) {
        let mut i = 0;
        while i < self.check_clear{
            self.trail[i].draw();
            i += 1;
        }
    }

}

pub struct GameOver {
    screen: [[char; BUFFER_WIDTH]; BUFFER_HEIGHT],
    points: usize,
    over: bool
}

impl GameOver {
    pub fn new(map: &str) -> Self {
        let mut screen = [[' '; BUFFER_WIDTH]; BUFFER_HEIGHT];
        for (row, chars) in map.split('\n').enumerate() {
            for (col, value) in chars.char_indices() {
                screen[row][col] = value;
            }
        }
        Self {screen, points: 0, over: true}
    }

    pub fn draw(&self) {
        for row in 0..self.screen.len() {
            for col in 0..self.screen[row].len() {
                plot(self.screen[row][col], col, row, ColorCode::new(Color::White, Color::Black));
            }
        }
        if self.over{
            plot_num(self.points.try_into().unwrap(), 41, 15 , ColorCode::new(Color::White, Color::Black));
        }
    }
}