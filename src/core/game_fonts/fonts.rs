pub enum GameFonts {
    ComfortaaBold,
    ComfortaaMedium,
    PressStart2P,
}

impl GameFonts {
    pub fn font_path(self) -> &'static str {
        match self {
            GameFonts::ComfortaaBold => "fonts/Comfortaa Font/static/Comfortaa-Bold.ttf",
            GameFonts::ComfortaaMedium => "fonts/Comfortaa Font/static/Comfortaa-Medium.ttf",
            GameFonts::PressStart2P => "fonts/Press Start 2P/PressStart2P-Regular.ttf",
        }
    }
}
