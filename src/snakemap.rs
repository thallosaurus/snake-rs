use crate::{
    enums::{Direction, VecState},
    utils,
};

use wasm_bindgen::prelude::*;
extern crate js_sys;
use js_sys::Math::{floor, random};

#[derive(Debug, Copy, Clone)]
struct SnakePart {
    x: u32,
    y: u32,
}

#[derive(Debug, Clone)]
#[wasm_bindgen]
pub struct SnakeMap {
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    snackX: u32,
    snackY: u32,
    direction: Direction,
    snake: Vec<SnakePart>,
    cells: Vec<VecState>,
}

#[wasm_bindgen]
impl SnakeMap {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn new(w: u32, h: u32) -> SnakeMap {
        utils::set_panic_hook();
        let x = 12;
        let y = 12;
        let dir = Direction::UP;

        let cells = (0..w * h).map(|i| VecState::OFF).collect();

        SnakeMap {
            width: w,
            height: h,
            direction: dir,
            x: x,
            y: y,
            snackX: 0,
            snackY: 0,
            cells: cells,
            snake: (0..4)
                .map(|i| {
                    return match dir {
                        UP => SnakePart { x: x, y: y - i },
                        DOWN => SnakePart { x: x, y: y + i },
                        LEFT => SnakePart { x: x - i, y: y },
                        RIGHT => SnakePart { x: x + i, y: y },
                    };
                })
                .collect(),
        }
    }

    fn spawn_snack(&mut self) {
        self.snackX = floor(random() * self.width as f64) as u32;
        self.snackY = floor(random() * self.height as f64) as u32;
    }

    pub fn tick(&mut self) {
        match self.direction {
            Direction::UP => {
                if self.y == 0 {
                    self.y = self.height - 1;
                } else {
                    self.y -= 1;
                }
            }
            Direction::DOWN => self.y = (self.y + 1) % self.height,
            Direction::LEFT => {
                if self.x == 0 {
                    self.x = self.width - 1;
                } else {
                    self.x -= 1 % self.width;
                }
            }
            Direction::RIGHT => self.x = (self.x + 1) % self.width,
        };

        //let s = self.snake.iter().find(|&(i, part)| part.x == self.x && part.y == self.y && i != 0);

        let s = self.snake.iter().enumerate().find(|&(i, part)| part.x == self.x && part.y == self.y && i != 0);

/*        match s {
            Some(p) => alert("You died"),
            _ => {}
        }*/

        let mut snakeparts = self.snake.clone();

        //unwrapping is okay here, because it will always have a length of > 1
        let mut part = snakeparts.pop().unwrap();

        part.x = self.x;
        part.y = self.y;

        snakeparts.insert(0, part);

        if part.x == self.snackX && part.y == self.snackY {
            self.spawn_snack();
            let cloned_part = part.clone();
            snakeparts.insert(snakeparts.len(), cloned_part);
        }

        self.snake = snakeparts;

        for y in 0..self.height {
            for x in 0..self.width {
                let snake = self.snake.iter().find(|part| part.x == x && part.y == y);

                let index = self.get_index(y, x);

                match snake {
                    Some(_p) => {
                        self.cells[index] = VecState::ON;
                    }
                    None => {
                        if x == self.snackX && y == self.snackY {
                            self.cells[index] = VecState::ON;
                        } else {
                            self.cells[index] = VecState::OFF;
                        }
                    }
                };
            }
        }
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn buffer_address(&self) -> *const VecState {
        self.cells.as_ptr()
    }

    pub fn move_up(&mut self) {
        if self.direction != Direction::DOWN {
            self.direction = Direction::UP;
        }
    }

    pub fn move_down(&mut self) {
        if self.direction != Direction::UP {
            self.direction = Direction::DOWN;
        }
    }

    pub fn move_left(&mut self) {
        if self.direction != Direction::RIGHT {
            self.direction = Direction::LEFT;
        }
    }

    pub fn move_right(&mut self) {
        if self.direction != Direction::LEFT {
            self.direction = Direction::RIGHT;
        }
    }
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}