pub enum GameFonts {
    HyperspaceBold,
    ComfortaaBold,
}

impl GameFonts {
    pub fn font_path(self) -> &'static str {
        match self {
            GameFonts::HyperspaceBold => "fonts/Hyperspace Font/Hyperspace Bold.otf",
            GameFonts::ComfortaaBold => "fonts/Comfortaa Font/static/Comfortaa-Bold.ttf",
        }
    }
}
