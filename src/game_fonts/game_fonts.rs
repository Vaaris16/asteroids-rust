pub enum GameFonts {
    HyperspaceBold,
    ComfortaaBold,
    ComfortaaMedium,
    VectorBattle,
}

impl GameFonts {
    pub fn font_path(self) -> &'static str {
        match self {
            GameFonts::HyperspaceBold => "fonts/Hyperspace Font/Hyperspace Bold.otf",
            GameFonts::ComfortaaBold => "fonts/Comfortaa Font/static/Comfortaa-Bold.ttf",
            GameFonts::ComfortaaMedium => "fonts/Comfortaa Font/static/Comfortaa-Medium.ttf",
            GameFonts::VectorBattle => "fonts/Vector Battle Font/Vectorb.ttf",
        }
    }
}
