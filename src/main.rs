// main.rs

use ggez::{ContextBuilder, event, GameResult};
use ggez::event::KeyCode;

use gametest::{GameState, handle_input, update, render};

struct MainState {
    game_state: GameState,
}

impl MainState {
    fn new() -> GameResult<Self> {
        let game_state = GameState::new();
        Ok(MainState { game_state })
    }
}
impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut ggez::Context) -> GameResult {
        handle_input(&mut self.game_state, _ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> GameResult {
        render(&self.game_state, ctx)
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ContextBuilder::new("Simple Game", "Your Name")
        .build()?;
    let main_state = MainState::new()?;
    event::run(ctx, event_loop, main_state)
}
