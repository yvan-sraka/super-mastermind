pub mod colors {
    use std::fmt;

    use rand::Rng;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum Color {
        BLUE,
        RED,
        PURPLE,
        GREEN,
        YELLOW,
        ORANGE,
        CYAN,
        WHITE,
    }
    impl fmt::Display for Color {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let description = match *self {
                Color::BLUE => "B",
                Color::RED => "R",
                Color::PURPLE => "P",
                Color::GREEN => "G",
                Color::YELLOW => "Y",
                Color::ORANGE => "O",
                Color::CYAN => "C",
                Color::WHITE => "W",
            };
            f.write_str(description)
        }
    }
}

