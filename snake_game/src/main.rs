mod sprites;

use ggez::{
    Context, ContextBuilder, GameResult,
    graphics::{self, Color},
    event::{self, EventHandler}
};

fn main () {
    let (mut ctx, event_loop) = ContextBuilder::new("snake", "Jot4ce")
        .build()
        .expect("Error al crear el contexto");
    let snake_game = SnakeGame::new(&mut ctx);
    event::run(ctx, event_loop, snake_game);
}

#[allow(dead_code)]
struct SnakeGame {
    snake_color: Color
}

impl SnakeGame {
    fn new (_ctx: &mut Context) -> Self {
        SnakeGame {
            snake_color: Color::new(0.0, 0.0, 0.0, 1.0)
        }
    }
}

impl EventHandler for SnakeGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Actualizar el juego aquí...
         Ok(())
    }
    
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::WHITE);
        let rectangle = graphics::Rect::new(50.0, 50.0, 100.0, 100.0);
        let rectangle_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rectangle, self.snake_color)?;
        
        canvas.draw(&rectangle_mesh, graphics::DrawParam::default());
        canvas.finish(ctx)
    }
 }