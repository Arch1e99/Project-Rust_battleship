use macroquad::prelude::*;
use std::collections::HashSet;

use crate::assets::Assets;
use crate::constants::*;
use crate::game_state::Field;

pub struct Board {
    pub grid: [[Field; BOARD_SIZE]; BOARD_SIZE],
    pub x_pos: f32,
    pub y_pos: f32,
    pub potential_targets: Vec<(usize, usize)>,
    pub hunt_mode_targets: Vec<(usize, usize)>,
}

impl Board {
    pub fn new(x_pos: f32, y_pos: f32) -> Self {
        let mut targets = Vec::new();
        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                targets.push((r, c));
            }
        }

        let len = targets.len();
        for i in (1..len).rev() {
            let j = rand::gen_range(0, i + 1);
            targets.swap(i, j);
        }

        Self {
            grid: [[Field::Water; BOARD_SIZE]; BOARD_SIZE],
            x_pos,
            y_pos,
            potential_targets: targets,
            hunt_mode_targets: Vec::new(),
        }
    }

    pub fn is_valid_placement(&self, r: usize, c: usize, len: usize, horizontal: bool) -> bool {
        if horizontal {
            if c + len > BOARD_SIZE {
                return false;
            }
        } else {
            if r + len > BOARD_SIZE {
                return false;
            }
        }

        let r_start = r.saturating_sub(1);
        let c_start = c.saturating_sub(1);
        let r_end = if horizontal {
            (r + 1).min(BOARD_SIZE - 1)
        } else {
            (r + len).min(BOARD_SIZE - 1)
        };
        let c_end = if horizontal {
            (c + len).min(BOARD_SIZE - 1)
        } else {
            (c + 1).min(BOARD_SIZE - 1)
        };

        for i in r_start..=r_end {
            for j in c_start..=c_end {
                if self.grid[i][j] != Field::Water {
                    return false;
                }
            }
        }
        true
    }

    pub fn place_ship(&mut self, r: usize, c: usize, len: usize, horizontal: bool) {
        for i in 0..len {
            if horizontal {
                self.grid[r][c + i] = Field::Ship;
            } else {
                self.grid[r + i][c] = Field::Ship;
            }
        }
    }

    pub fn randomize_ships(&mut self) {
        let ships = [4, 3, 3, 2, 2, 2, 1, 1, 1, 1];
        for &len in &ships {
            let mut placed = false;
            let mut attempts = 0;
            while !placed && attempts < 1000 {
                let horizontal = rand::gen_range(0, 2) == 0;
                let r = rand::gen_range(0, BOARD_SIZE);
                let c = rand::gen_range(0, BOARD_SIZE);
                if self.is_valid_placement(r, c, len, horizontal) {
                    self.place_ship(r, c, len, horizontal);
                    placed = true;
                }
                attempts += 1;
            }
        }
    }

    pub fn check_and_mark_sunk(&mut self, start_r: usize, start_c: usize) -> bool {
        let mut ship_parts = Vec::new();
        let mut stack = vec![(start_r, start_c)];
        let mut visited = HashSet::new();

        while let Some((r, c)) = stack.pop() {
            if visited.contains(&(r, c)) {
                continue;
            }
            visited.insert((r, c));
            ship_parts.push((r, c));

            let neighbors = [(0, 1), (0, -1), (1, 0), (-1, 0)];
            for (dr, dc) in neighbors {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr >= 0 && nr < BOARD_SIZE as i32 && nc >= 0 && nc < BOARD_SIZE as i32 {
                    let nr = nr as usize;
                    let nc = nc as usize;
                    match self.grid[nr][nc] {
                        Field::Ship => return false,
                        Field::Hit => stack.push((nr, nc)),
                        _ => {}
                    }
                }
            }
        }

        for (r, c) in &ship_parts {
            self.grid[*r][*c] = Field::Sunk;
        }
        for (r, c) in &ship_parts {
            let r = *r as i32;
            let c = *c as i32;
            for dr in -1..=1 {
                for dc in -1..=1 {
                    let nr = r + dr;
                    let nc = c + dc;
                    if nr >= 0 && nr < BOARD_SIZE as i32 && nc >= 0 && nc < BOARD_SIZE as i32 {
                        let nr = nr as usize;
                        let nc = nc as usize;
                        if self.grid[nr][nc] == Field::Water {
                            self.grid[nr][nc] = Field::Miss;
                        }
                    }
                }
            }
        }
        true
    }

    pub fn all_sunk(&self) -> bool {
        !self.grid.iter().flatten().any(|&f| f == Field::Ship)
    }

    pub fn get_hover(&self) -> Option<(usize, usize)> {
        let (mx, my) = mouse_position();
        let col = ((mx - self.x_pos) / CELL_SIZE) as i32;
        let row = ((my - self.y_pos) / CELL_SIZE) as i32;
        if col >= 0 && col < BOARD_SIZE as i32 && row >= 0 && row < BOARD_SIZE as i32 {
            Some((row as usize, col as usize))
        } else {
            None
        }
    }

    pub fn draw(&self, hide_ships: bool, _assets: &Assets) {
        for i in 0..BOARD_SIZE {
            let letter = ((b'A' + i as u8) as char).to_string();
            draw_text(
                &letter,
                self.x_pos + i as f32 * CELL_SIZE + 10.0,
                self.y_pos - 5.0,
                20.0,
                BLACK,
            );
            let number = (i + 1).to_string();
            draw_text(
                &number,
                self.x_pos - 25.0,
                self.y_pos + i as f32 * CELL_SIZE + 22.0,
                20.0,
                BLACK,
            );
        }

        for r in 0..BOARD_SIZE {
            for c in 0..BOARD_SIZE {
                let x = self.x_pos + c as f32 * CELL_SIZE;
                let y = self.y_pos + r as f32 * CELL_SIZE;
                let field = self.grid[r][c];

                let color = match field {
                    Field::Water => COL_WATER,
                    Field::Ship => {
                        if hide_ships {
                            COL_WATER
                        } else {
                            COL_SHIP
                        }
                    }
                    Field::Hit => COL_HIT,
                    Field::Sunk => COL_SUNK,
                    Field::Miss => COL_WATER,
                };

                draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, color);
                draw_rectangle_lines(x, y, CELL_SIZE, CELL_SIZE, 1.0, WHITE);

                match field {
                    Field::Hit => {
                        draw_line(
                            x + 5.,
                            y + 5.,
                            x + CELL_SIZE - 5.,
                            y + CELL_SIZE - 5.,
                            3.0,
                            BLACK,
                        );
                        draw_line(
                            x + CELL_SIZE - 5.,
                            y + 5.,
                            x + 5.,
                            y + CELL_SIZE - 5.,
                            3.0,
                            BLACK,
                        );
                    }
                    Field::Sunk => {
                        draw_line(
                            x + 5.,
                            y + 5.,
                            x + CELL_SIZE - 5.,
                            y + CELL_SIZE - 5.,
                            3.0,
                            RED,
                        );
                        draw_line(
                            x + CELL_SIZE - 5.,
                            y + 5.,
                            x + 5.,
                            y + CELL_SIZE - 5.,
                            3.0,
                            RED,
                        );
                    }
                    Field::Miss => {
                        draw_circle(x + CELL_SIZE / 2.0, y + CELL_SIZE / 2.0, 6.0, COL_MISS);
                    }
                    _ => {}
                }
            }
        }
    }
}
