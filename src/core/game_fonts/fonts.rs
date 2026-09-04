pub enum GameFonts {
    ComfortaaBold,
    ComfortaaMedium,
}

impl GameFonts {
    pub fn font_path(self) -> &'static str {
        match self {
            GameFonts::ComfortaaBold => "fonts/Comfortaa Font/static/Comfortaa-Bold.ttf",
            GameFonts::ComfortaaMedium => "fonts/Comfortaa Font/static/Comfortaa-Medium.ttf",
        }
    }
}
