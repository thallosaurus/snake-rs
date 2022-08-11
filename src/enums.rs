use wasm_bindgen::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
#[wasm_bindgen]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
#[wasm_bindgen]
pub enum VecState {
    ON = 1,
    OFF = 0,
}