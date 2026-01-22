use macroquad::prelude::*;

pub struct Assets {
    pub background: Option<Texture2D>,
}

impl Assets {
    pub async fn load() -> Self {
        let texture_result = load_texture("assets/background.png").await;

        let background = match texture_result {
            Ok(texture) => {
                println!("SUKCES");
                Some(texture)
            }
            Err(_err) => {
                println!("BLAD");
                None
            }
        };

        Self { background }
    }
}
