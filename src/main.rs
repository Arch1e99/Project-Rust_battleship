mod assets;
mod board;
mod constants;
mod game_state;

use macroquad::prelude::*;

use crate::assets::Assets;
use crate::board::Board;
use crate::constants::*;
use crate::game_state::{Field, GameState};

fn draw_ship_panel(ships_queue: &[usize]) {
    let start_x = 20.0;
    let start_y = MARGIN_TOP + (BOARD_SIZE as f32 * CELL_SIZE) + 30.0;

    draw_text("Statki do postawienia:", start_x, start_y, 24.0, DARKGRAY);

    let mut current_x = start_x;
    let y = start_y + 20.0;

    for &len in ships_queue {
        let width = len as f32 * 20.0;
        draw_rectangle(current_x, y, width, 15.0, DARKGRAY);
        draw_rectangle_lines(current_x, y, width, 15.0, 1.0, BLACK);
        for part in 1..len {
            draw_line(
                current_x + part as f32 * 20.0,
                y,
                current_x + part as f32 * 20.0,
                y + 15.0,
                1.0,
                BLACK,
            );
        }
        current_x += width + 10.0;
    }

    if ships_queue.is_empty() {
        draw_text("Gotowe! Nacisnij ENTER.", start_x, y + 25.0, 30.0, GREEN);
    }
}

#[macroquad::main("Battleship")]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);
    request_new_screen_size(900.0, 700.0);

    let assets = Assets::load().await;

    let mut player_board = Board::new(MARGIN_LEFT, MARGIN_TOP);
    let mut enemy_board = Board::new(
        MARGIN_LEFT + BOARD_SIZE as f32 * CELL_SIZE + GAP,
        MARGIN_TOP,
    );

    let ships_to_place_config = vec![4, 3, 3, 2, 2, 2, 1, 1, 1, 1];
    let mut ships_queue = ships_to_place_config.clone();
    let mut current_orientation_hor = true;

    let mut state = GameState::Placement;
    let mut msg = String::from("Ustaw swoja flote.");
    let mut turn_timer = 0.0;

    loop {
        if let Some(bg) = &assets.background {
            draw_texture_ex(
                bg,
                0.0,
                0.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(vec2(screen_width(), screen_height())),
                    ..Default::default()
                },
            );
        } else {
            clear_background(LIGHTGRAY);
        }

        match state {
            GameState::Placement => {
                if let Some(&current_len) = ships_queue.first() {
                    if is_key_pressed(KeyCode::R) {
                        current_orientation_hor = !current_orientation_hor;
                    }
                    let orient_txt = if current_orientation_hor {
                        "POZIOMO"
                    } else {
                        "PIONOWO"
                    };
                    msg = format!(
                        "Ustawiasz: {} masztowiec ({}). [R] - Obrot",
                        current_len, orient_txt
                    );

                    if let Some((r, c)) = player_board.get_hover() {
                        let valid = player_board.is_valid_placement(
                            r,
                            c,
                            current_len,
                            current_orientation_hor,
                        );
                        let color = if valid {
                            COL_GHOST
                        } else {
                            Color::new(1.0, 0.0, 0.0, 0.5)
                        };

                        for i in 0..current_len {
                            let (dr, dc) = if current_orientation_hor {
                                (0, i)
                            } else {
                                (i, 0)
                            };
                            if r + dr < BOARD_SIZE && c + dc < BOARD_SIZE {
                                let x = player_board.x_pos + (c + dc) as f32 * CELL_SIZE;
                                let y = player_board.y_pos + (r + dr) as f32 * CELL_SIZE;
                                draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, color);
                            }
                        }

                        if is_mouse_button_pressed(MouseButton::Left) && valid {
                            player_board.place_ship(r, c, current_len, current_orientation_hor);
                            ships_queue.remove(0);
                        }
                    }
                } else {
                    msg = String::from("Statki ustawione. ENTER aby zaczac.");
                    if is_key_pressed(KeyCode::Enter) {
                        enemy_board.randomize_ships();
                        state = GameState::PlayerTurn;
                        msg = String::from("Twoj ruch! Ognia!");
                    }
                }
            }
            GameState::PlayerTurn => {
                if let Some((r, c)) = enemy_board.get_hover() {
                    let x = enemy_board.x_pos + c as f32 * CELL_SIZE;
                    let y = enemy_board.y_pos + r as f32 * CELL_SIZE;
                    draw_rectangle_lines(x, y, CELL_SIZE, CELL_SIZE, 3.0, ORANGE);

                    if is_mouse_button_pressed(MouseButton::Left) {
                        let field = enemy_board.grid[r][c];
                        if field == Field::Water || field == Field::Ship {
                            if field == Field::Ship {
                                enemy_board.grid[r][c] = Field::Hit;
                                if enemy_board.check_and_mark_sunk(r, c) {
                                    msg = String::from("ZATOPIONY! Strzelaj dalej.");
                                } else {
                                    msg = String::from("TRAFIONY! Strzelaj dalej.");
                                }
                                if enemy_board.all_sunk() {
                                    state = GameState::GameOver("ZWYCIESTWO!".to_string());
                                }
                            } else {
                                enemy_board.grid[r][c] = Field::Miss;
                                msg = String::from("Pudlo. Tura komputera.");
                                state = GameState::ComputerTurn;
                                turn_timer = 0.8;
                            }
                        }
                    }
                }
            }
            GameState::ComputerTurn => {
                turn_timer -= get_frame_time();
                if turn_timer <= 0.0 {
                    let (r, c) = if let Some(target) = player_board.hunt_mode_targets.pop() {
                        target
                    } else if let Some(target) = player_board.potential_targets.pop() {
                        target
                    } else {
                        (0, 0)
                    };

                    let field = player_board.grid[r][c];
                    if field != Field::Hit && field != Field::Miss && field != Field::Sunk {
                        if field == Field::Ship {
                            player_board.grid[r][c] = Field::Hit;
                            msg = String::from("Komputer TRAFIL!");
                            if player_board.check_and_mark_sunk(r, c) {
                                msg = String::from("Komputer ZATOPIL twoj statek!");
                                player_board.hunt_mode_targets.clear();
                            } else {
                                let neighbors = [(0, 1), (0, -1), (1, 0), (-1, 0)];
                                for (dr, dc) in neighbors {
                                    let nr = r as i32 + dr;
                                    let nc = c as i32 + dc;
                                    if nr >= 0
                                        && nr < BOARD_SIZE as i32
                                        && nc >= 0
                                        && nc < BOARD_SIZE as i32
                                    {
                                        player_board
                                            .hunt_mode_targets
                                            .push((nr as usize, nc as usize));
                                    }
                                }
                            }
                            if player_board.all_sunk() {
                                state = GameState::GameOver("PORAZKA...".to_string());
                            }
                            turn_timer = 0.8;
                        } else {
                            player_board.grid[r][c] = Field::Miss;
                            msg = String::from("Komputer spudlowal. Twoja tura.");
                            state = GameState::PlayerTurn;
                        }
                    }
                }
            }
            GameState::GameOver(_) => {
                if is_key_pressed(KeyCode::Enter) {
                    player_board = Board::new(MARGIN_LEFT, MARGIN_TOP);
                    enemy_board = Board::new(
                        MARGIN_LEFT + BOARD_SIZE as f32 * CELL_SIZE + GAP,
                        MARGIN_TOP,
                    );
                    ships_queue = ships_to_place_config.clone();
                    state = GameState::Placement;
                    msg = String::from("Ustaw swoja flote.");
                }
            }
        }

        let screen_w = screen_width();
        let title = "BATTLESHIP RUST";
        let title_width = measure_text(title, None, 42, 1.0).width;
        draw_text(title, (screen_w - title_width) / 2.0, 55.0, 42.0, DARKGRAY);

        draw_text("TY", player_board.x_pos, 80.0, 32.0, BLACK);
        let enemy_label_x = enemy_board.x_pos + 40.0;
        draw_text("KOMPUTER", enemy_label_x, 80.0, 32.0, BLACK);

        player_board.draw(false, &assets);
        let hide_enemy = !matches!(state, GameState::GameOver(_));
        enemy_board.draw(hide_enemy, &assets);

        if matches!(state, GameState::Placement) {
            draw_ship_panel(&ships_queue);
        }

        draw_text(&msg, MARGIN_LEFT, 550.0, 30.0, BLACK);

        if let GameState::GameOver(winner_msg) = &state {
            draw_rectangle(100.0, 250.0, 700.0, 200.0, Color::new(0.0, 0.0, 0.0, 0.9));
            let col = if winner_msg.contains("ZWYCIESTWO") {
                GREEN
            } else {
                RED
            };
            draw_text(winner_msg, 320.0, 330.0, 50.0, col);
            draw_text(
                "Nacisnij ENTER aby zagrac ponownie",
                280.0,
                380.0,
                20.0,
                WHITE,
            );
        }

        next_frame().await;
    }
}
