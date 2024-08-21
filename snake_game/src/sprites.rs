use ggez::graphics;

enum Sprites {
    SnakeHead,
    SnakeBody,
    SnakeTail,
    SnakeBodyBottomRight,
    SnakeBodyBottomLeft,
    SnakeBodyTopRight,
    SnakeBodyTopLeft,
}

impl Sprites {
    fn get(&self) -> (f32, f32, f32, f32) {
        match self {
            Sprites::SnakeHead => (1.0, 1.0, 40.0, 40.0),
            Sprites::SnakeBody => (85.0, 85.0, 40.0, 40.0),
            Sprites::SnakeTail => (43.0, 85.0, 40.0, 40.0),
            Sprites::SnakeBodyBottomRight => (43.0, 0.0, 40.0, 40.0),
            Sprites::SnakeBodyBottomLeft => (85.0, 0.0, 40.0, 40.0),
            Sprites::SnakeBodyTopRight => (43.0, 43.0, 40.0, 40.0),
            Sprites::SnakeBodyTopLeft => (85.0, 43.0, 40.0, 40.0),
        }

    }
}