use bracket_lib::prelude::*;
use std::{time, thread};
use std::cmp::{min, max};
pub const SCREEN_WIDTH: usize = 100;
pub const SCREEN_HEIGHT: usize = 100;

#[derive(Copy, Clone, PartialEq)]
pub enum CellType {
    Floor,
    Wall,
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub cell_type: CellType,
    pub visited: bool,
    pub weight: f32,
}

pub struct State {
    pub cells: [Cell; SCREEN_HEIGHT * SCREEN_WIDTH],
    pub from_x: usize,
    pub from_y: usize,
    pub to_x: usize,
    pub to_y: usize,
    pub current_x: usize,
    pub current_y: usize,
}

impl State {
    pub fn new(from_x: usize, from_y: usize, to_x: usize, to_y: usize) -> Self {
        State {
            cells: [
                Cell { 
                    cell_type: CellType::Floor,
                    visited: false,
                    weight: f32::MAX,
                }; 
                SCREEN_HEIGHT * SCREEN_WIDTH
            ],
            from_x,
            from_y,
            to_x,
            to_y,
            current_x: from_x,
            current_y: from_y,
        }
    }

    pub fn set_type_at(&mut self, x: usize, y: usize, cell_type: CellType) {
        self.cells[y * SCREEN_WIDTH + x].cell_type = cell_type;
    }

    pub fn set_weight_at(&mut self, x: usize, y: usize, weight: f32) {
        self.cells[y * SCREEN_WIDTH + x].weight = weight;
    }

    pub fn set_visited_at(&mut self, x: usize, y: usize, visited: bool) {
        self.cells[y * SCREEN_WIDTH + x].visited = visited;
    }

    pub fn get_at(&mut self, x: usize, y: usize) -> &mut Cell {
        &mut self.cells[y * SCREEN_WIDTH + x]
    }

    pub fn get_weight_at(&mut self, x: usize, y: usize) -> f32 {
        self.cells[y * SCREEN_WIDTH + x].weight
    }

    pub fn get_floor_neighbours(&mut self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut indices = vec![];
        for i in max(0, x as i32 - 1)..min(SCREEN_WIDTH as i32, x as i32 + 2) {
            for j in max(0, y as i32 - 1)..min(SCREEN_HEIGHT as i32, y as i32 + 2) {
                if x as i32 != i || y as i32 != j {
                    let cell = self.get_at(i as usize, j as usize);
                    if !cell.visited && cell.cell_type == CellType::Floor {
                        indices.push((i as usize, j as usize));
                    }
                }
            }
        }
        indices
    }

    pub fn render(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        for (index, cell) in self.cells.iter().enumerate() {
            let (x, y) = coords_from_index(index);
            match cell.cell_type {
                CellType::Floor => {
                    let cell_coords = coords_from_index(index);
                    let mut color = BLACK;
                    if cell.visited {
                        color = YELLOW;
                    } else if cell_coords.0 == self.current_x && cell_coords.1 == self.current_y {
                        color = RED;
                    }
                    ctx.set(x, y, WHITE, color, to_cp437('.'))
                },
                CellType::Wall => ctx.set(x, y, GREEN, BLACK, to_cp437('#')),
            }
        }
    }

    pub fn iterate(&mut self) {
        let current_x = self.current_x;
        let current_y = self.current_y;
        if current_x == self.to_x && current_y == self.to_y {
            return;
        }
        self.set_visited_at(current_x, current_y, true);
        let neighbours = self.get_floor_neighbours(current_x, current_y);
        let current_cell_weight = self.get_weight_at(current_x, current_y);
        for (x, y) in neighbours {
            let cell = self.get_at(x, y);
            let distance = if current_x == x || current_y == y { 1.0 } else { 1.4 };
            let new_weight = current_cell_weight + distance;
            if new_weight < cell.weight {
                cell.weight = new_weight;
            }
        }
        let cell_index_to_go = self.get_lowest_weight_cell_index();
        let (go_to_x, go_to_y) = coords_from_index(cell_index_to_go);
        self.current_x = go_to_x;
        self.current_y = go_to_y;
    }

    pub fn get_lowest_weight_cell_index(&self) -> usize {
        let mut current_weight = f32::MAX;
        let mut result_index = 0;
        for (index, cell) in self.cells.iter().enumerate() {
            if cell.weight < current_weight && !cell.visited {
                result_index = index;
                current_weight = cell.weight;
            }
        };

        result_index
    }
}

pub fn to_index(x: usize, y: usize) -> usize {
    y * SCREEN_WIDTH + x
}

pub fn coords_from_index(index: usize) -> (usize, usize) {
    (index % SCREEN_WIDTH, index / SCREEN_HEIGHT)
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        self.iterate();
        self.render(ctx);
        let wait = time::Duration::from_millis(10);
        thread::sleep(wait);
    }
}