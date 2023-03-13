use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}

pub fn in_bounds(point: Point) -> bool {
    point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
}

pub fn try_idx(point: Point) -> Option<usize> {
    if in_bounds(point) {
        Some(idx(point.x, point.y))
    } else {
        None
    }
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        in_bounds(point) && self.tiles[idx(point.x, point.y)] == TileType::Floor
    }
}
