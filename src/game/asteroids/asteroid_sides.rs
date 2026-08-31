use rand::RngExt;

// Enum to define the sides at which the asteroids can spawn.
pub enum Side {
    Top,
    Bottom,
    Right,
    Left,
}

impl Side {
    // Returns a random Side
    pub fn rand_side() -> Side {
        let mut rng = rand::rng();
        match rng.random_range(0..4) {
            0 => Side::Top,
            1 => Side::Bottom,
            2 => Side::Right,
            3 => Side::Left,
            _ => unreachable!(),
        }
    }
}
