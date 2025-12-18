use crate::constants::*;
use crate::game_state::Field;
use macroquad::prelude::*;
use std::collections::HashSet;

pub struct Board {
    pub grid: [[Field; BOARD_SIZE]; BOARD_SIZE],
    pub x_pos: f32,
    pub y_pos: f32,
    pub potential_targets: Vec<(usize, usize)>,
    pub hunt_mode_targets: Vec<(usize, usize)>,
}

impl Board {
    // inicjalizacja planszy i celow AI
    pub fn new(x_pos: f32, y_pos: f32) -> Self {
        // wypelnic targety i je przetasowac
        Self {
            grid: [[Field::Water; BOARD_SIZE]; BOARD_SIZE],
            x_pos,
            y_pos,
            potential_targets: Vec::new(),
            hunt_mode_targets: Vec::new(),
        }
    }

    // czy umieszczenie statku jest poprawne (granice + sąsiedztwo)
    pub fn is_valid_placement(&self, r: usize, c: usize, len: usize, horizontal: bool) -> bool {
        true
    }

    // umieszcza statek na planszy
    pub fn place_ship(&mut self, r: usize, c: usize, len: usize, horizontal: bool) {
        // TODO: wpisac statek do tablicy grid
    }

    // losowe ustawienie statkow (dla komputera)
    pub fn randomize_ships(&mut self) {
        // TODO: dodac logike losowego ustawiania calej floty
    }

    // sprawdza czy po trafieniu statek jest zatopiony i oznacza otoczenie
    pub fn check_and_mark_sunk(&mut self, start_r: usize, start_c: usize) -> bool {
        // TODO: Dodac logike rekurencyjna
        false
    }

    // czy wszystkie statki na planszy zatonely
    pub fn all_sunk(&self) -> bool {
        // TODO: dodac logike sprawdzajaca siatke
        false
    }

    // rysowanie planszy
    pub fn draw(&self, hide_ships: bool, title: &str) {
        // TODO: dodac logike rysowania siatki i pol
    }

    // Pobieranie współrzędnych pola pod myszką
    pub fn get_hover(&self) -> Option<(usize, usize)> {
        // TODO: dodac logike przeliczania pikseli na indeksy tablicy
        None
    }
}

// funkcja pomoc do rysowania panelu statkow
pub fn draw_ship_panel(ships_queue: &[usize]) {}
