use macroquad::prelude::*;

pub struct Assets {
    pub background: Option<Texture2D>,
}

impl Assets {
    pub async fn load() -> Self {
        let background = load_texture("assets/backsea.png").await.ok();
        Self { background }
    }
}
