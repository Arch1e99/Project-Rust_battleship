use macroquad::prelude::*;

mod board;
mod constants;
mod game_state;

use crate::board::{Board, draw_ship_panel};
use crate::constants::*;
use crate::game_state::{Field, GameState};

#[macroquad::main("Battleship")]
async fn main() {
    rand::srand(macroquad::miniquad::date::now() as u64);
    request_new_screen_size(900.0, 700.0);

    // plansze
    let mut player_board = Board::new(MARGIN_LEFT, MARGIN_TOP);
    let mut enemy_board = Board::new(
        MARGIN_LEFT + BOARD_SIZE as f32 * CELL_SIZE + GAP,
        MARGIN_TOP,
    );

    enemy_board.randomize_ships();

    // placement
    let ships_to_place_config = vec![4, 3, 3, 2, 2, 2, 1, 1, 1, 1];
    let mut ships_queue = ships_to_place_config.clone();
    let mut current_orientation_hor = true;

    let mut state = GameState::Placement;
    let mut msg = String::from("Ustaw swoje statki (R obrot)");
    let mut turn_timer = 0.0;

    loop {
        clear_background(LIGHTGRAY);

        // LOGIKA
        match state {
            GameState::Placement => {
                // obrot
                if is_key_pressed(KeyCode::R) {
                    current_orientation_hor = !current_orientation_hor;
                }

                // ghost preview + klik
                if let Some((r, c)) = player_board.get_hover() {
                    if !ships_queue.is_empty() {
                        let len = ships_queue[0];
                        let ok =
                            player_board.is_valid_placement(r, c, len, current_orientation_hor);
                        draw_ghost_ship(&player_board, r, c, len, current_orientation_hor, ok);

                        if ok && is_mouse_button_pressed(MouseButton::Left) {
                            player_board.place_ship(r, c, len, current_orientation_hor);
                            ships_queue.remove(0);

                            if ships_queue.is_empty() {
                                msg = "Statki ustawione. ENTER aby zaczac".to_string();
                            } else {
                                msg = format!("Ustaw statek dlugosci {} (R obrot)", ships_queue[0]);
                            }
                        }
                    }
                }

                // start gry
                if ships_queue.is_empty() && is_key_pressed(KeyCode::Enter) {
                    state = GameState::PlayerTurn;
                    msg = "Twoja tura: strzelaj w plansze komputera".to_string();
                }
            }

            GameState::PlayerTurn => {
                // hover na enemy + strzal
                if let Some((r, c)) = enemy_board.get_hover() {
                    let x = enemy_board.x_pos + c as f32 * CELL_SIZE;
                    let y = enemy_board.y_pos + r as f32 * CELL_SIZE;
                    draw_rectangle_lines(x, y, CELL_SIZE, CELL_SIZE, 3.0, ORANGE);

                    if is_mouse_button_pressed(MouseButton::Left) {
                        match enemy_board.grid[r][c] {
                            Field::Water => {
                                enemy_board.grid[r][c] = Field::Miss;
                                msg = "Pudlo! Tura komputera...".to_string();
                                state = GameState::ComputerTurn;
                                turn_timer = 0.8;
                            }
                            Field::Ship => {
                                enemy_board.grid[r][c] = Field::Hit;
                                let sunk = enemy_board.check_and_mark_sunk(r, c);
                                msg = if sunk {
                                    "ZATOPIONY! Strzelaj dalej".to_string()
                                } else {
                                    "TRAFIONY!".to_string()
                                };

                                if enemy_board.all_sunk() {
                                    state = GameState::GameOver("ZWYCIESTWO!".to_string());
                                }
                            }
                            _ => {
                                // TODO (final): krotszy dzwiek / komunikat "tu juz strzelales"
                            }
                        }
                    }
                }
            }

            GameState::ComputerTurn => {
                turn_timer -= get_frame_time();
                if turn_timer <= 0.0 {
                    // TODO (final): filtrowanie duplikatow, lepsze dobijanie kierunkowe

                    let target = pick_ai_target(&mut player_board);
                    if let Some((r, c)) = target {
                        match player_board.grid[r][c] {
                            Field::Water => {
                                player_board.grid[r][c] = Field::Miss;
                                msg = "Twoja tura".to_string();
                                state = GameState::PlayerTurn;
                            }
                            Field::Ship => {
                                player_board.grid[r][c] = Field::Hit;
                                let sunk = player_board.check_and_mark_sunk(r, c);

                                if !sunk {
                                    push_neighbors_to_hunt(&mut player_board, r, c);
                                } else {
                                    // TODO (final): czyscic hunt_mode i trzymac kierunek dobijania
                                    player_board.hunt_mode_targets.clear();
                                }

                                if player_board.all_sunk() {
                                    state = GameState::GameOver("PORAZKA...".to_string());
                                } else {
                                    msg = "Komputer trafil...".to_string();
                                    turn_timer = 0.6;
                                }
                            }
                            _ => {
                                msg = "(AI) blad wyboru pola".to_string();
                                state = GameState::PlayerTurn;
                            }
                        }
                    } else {
                        // brak pol (teoretycznie niemozliwe)
                        state = GameState::PlayerTurn;
                    }
                }
            }

            GameState::GameOver(ref winner_msg) => {
                // TODO (final): ladny overlay + kolory zalezne od wyniku
                msg = format!("Koniec gry: {} ([ENTER] to restart)", winner_msg);

                if is_key_pressed(KeyCode::Enter) {
                    // reset
                    player_board = Board::new(MARGIN_LEFT, MARGIN_TOP);
                    enemy_board = Board::new(
                        MARGIN_LEFT + BOARD_SIZE as f32 * CELL_SIZE + GAP,
                        MARGIN_TOP,
                    );
                    enemy_board.randomize_ships();

                    ships_queue = ships_to_place_config.clone();
                    current_orientation_hor = true;
                    state = GameState::Placement;
                    msg = "Ustaw swoje statki (R obrot)".to_string();
                    turn_timer = 0.0;
                }
            }
        }

        // RYSOWANIE
        draw_text("BATTLESHIP", 340.0, 50.0, 44.0, DARKGRAY);

        player_board.draw(false, "TY");
        enemy_board.draw(true, "KOMPUTER");

        if state == GameState::Placement {
            draw_ship_panel(&ships_queue);
        }

        draw_text(&msg, MARGIN_LEFT, 585.0, 28.0, BLACK);

        next_frame().await;
    }
}

fn draw_ghost_ship(board: &Board, r: usize, c: usize, len: usize, horizontal: bool, ok: bool) {
    let ghost = if ok {
        COL_GHOST
    } else {
        Color::new(1.0, 0.0, 0.0, 0.35)
    };

    for i in 0..len {
        let rr = if horizontal { r } else { r + i };
        let cc = if horizontal { c + i } else { c };

        if rr >= BOARD_SIZE || cc >= BOARD_SIZE {
            break;
        }

        let x = board.x_pos + cc as f32 * CELL_SIZE;
        let y = board.y_pos + rr as f32 * CELL_SIZE;
        draw_rectangle(x, y, CELL_SIZE, CELL_SIZE, ghost);
        draw_rectangle_lines(x, y, CELL_SIZE, CELL_SIZE, 2.0, WHITE);
    }
}

fn pick_ai_target(board: &mut Board) -> Option<(usize, usize)> {
    // najpierw hunt_mode (jesli cos tam jest)
    while let Some((r, c)) = board.hunt_mode_targets.pop() {
        if matches!(board.grid[r][c], Field::Water | Field::Ship) {
            return Some((r, c));
        }
    }

    // potem lista wszystkich pol
    while let Some((r, c)) = board.potential_targets.pop() {
        if matches!(board.grid[r][c], Field::Water | Field::Ship) {
            return Some((r, c));
        }
    }

    None
}

fn push_neighbors_to_hunt(board: &mut Board, r: usize, c: usize) {
    let dirs = [(0_i32, 1_i32), (0, -1), (1, 0), (-1, 0)];
    for (dr, dc) in dirs {
        let nr = r as i32 + dr;
        let nc = c as i32 + dc;
        if nr >= 0 && nr < BOARD_SIZE as i32 && nc >= 0 && nc < BOARD_SIZE as i32 {
            let (nr, nc) = (nr as usize, nc as usize);
            if matches!(board.grid[nr][nc], Field::Water | Field::Ship) {
                board.hunt_mode_targets.push((nr, nc));
            }
        }
    }
}
