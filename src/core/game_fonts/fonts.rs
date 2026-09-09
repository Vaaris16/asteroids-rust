pub enum GameFonts {
    PressStart2P,
}

const PRESS_START_2P_PATH: &str = "fonts/Press Start 2P/PressStart2P-Regular.ttf";

impl GameFonts {
    pub fn font_path(self) -> &'static str {
        match self {
            GameFonts::PressStart2P => PRESS_START_2P_PATH,
        }
    }
}
