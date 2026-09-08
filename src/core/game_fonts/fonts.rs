pub enum GameFonts {
    PressStart2P,
}

impl GameFonts {
    pub fn font_path(self) -> &'static str {
        match self {
            GameFonts::PressStart2P => "fonts/Press Start 2P/PressStart2P-Regular.ttf",
        }
    }
}
