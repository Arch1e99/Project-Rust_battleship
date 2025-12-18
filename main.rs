use macroquad::prelude::*;

mod board;
mod constants;
mod game_state;

use crate::board::{Board, draw_ship_panel};
use crate::constants::*;
use crate::game_state::{Field, GameState};

#[macroquad::main("Battleship - Szkielet")]
async fn main() {
    // inicjalizacja (losowanie i okno)
    rand::srand(macroquad::miniquad::date::now() as u64);
    request_new_screen_size(900.0, 700.0);

    // inicjalizacja stanow
    let mut player_board = Board::new(MARGIN_LEFT, MARGIN_TOP);
    let mut enemy_board = Board::new(
        MARGIN_LEFT + BOARD_SIZE as f32 * CELL_SIZE + GAP,
        MARGIN_TOP,
    );

    // konfiguracja do stawiania
    let ships_to_place_config = vec![4, 3, 3, 2, 2, 2, 1, 1, 1, 1];
    let mut ships_queue = ships_to_place_config.clone();
    let mut current_orientation_hor = true;

    let mut state = GameState::Placement;
    let mut msg = String::from("Trwa inicjalizacja...");
    let mut turn_timer = 0.0;

    loop {
        clear_background(LIGHTGRAY);

        match state {
            GameState::Placement => {
                // TODO: logika ustawiania (mysz, [R], [ENTER])
                msg = String::from("Trwa ustawianie statków...");
            }

            GameState::PlayerTurn => {
                // TODO: logika tury gracza (strzelanie, sprawdzanie wygranej)
                msg = String::from("Tura Gracza!");
            }

            GameState::ComputerTurn => {
                turn_timer -= get_frame_time();
                if turn_timer <= 0.0 {
                    // TODO: logika tury AI (wybor celu, strzal, zmiana tury)
                    msg = String::from("Tura Komputera...");
                }
            }

            GameState::GameOver(ref winner_msg) => {
                // TODO: logika konca gry i resetu
                msg = format!("Koniec gry: {}", winner_msg);
            }
        }

        // RYSOWANIE (Szkielet)
        draw_text("BATTLESHIP RUST", 300.0, 50.0, 40.0, DARKGRAY);

        player_board.draw(false, "TY");
        enemy_board.draw(true, "KOMPUTER");

        if state == GameState::Placement {
            draw_ship_panel(&ships_queue);
        }

        draw_text(&msg, MARGIN_LEFT, 550.0, 30.0, BLACK);

        next_frame().await
    }
}
