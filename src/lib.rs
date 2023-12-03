// lib.rs

use ggez::{graphics, Context, GameResult};
use ggez::event::KeyCode;
use nalgebra as na;

#[derive(Debug, Copy, Clone)]
pub struct GameState {
    pub player_position: na::Point2<f32>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            player_position: na::Point2::new(300.0, 200.0),
        }
    }
}

// pub fn handle_input(game_state: &mut GameState, keycode: KeyCode) {
//     let speed = 5.0; // Adjust as needed

//     match keycode {
//         KeyCode::Up => game_state.player_position.y -= speed,
//         KeyCode::Down => game_state.player_position.y += speed,
//         KeyCode::Left => game_state.player_position.x -= speed,
//         KeyCode::Right => game_state.player_position.x += speed,
//         _ => (),
//     }
// }

pub fn handle_input(game_state: &mut GameState, ctx: &mut ggez::Context) {
    let speed = 5.0; // Adjust as needed

    // Check keyboard input
    if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Up) {
        game_state.player_position.y -= speed;
    }
    if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Left) {
        game_state.player_position.x -= speed;
    }
    if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Right) {
        game_state.player_position.x += speed;
    }
}    

pub fn update(game_state: &mut GameState, _: &mut Context) -> GameResult {
    Ok(())
}

pub fn render(game_state: &GameState, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::Color::BLACK);

    // Draw the player
    let player_rect = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::fill(),
        graphics::Rect::new(
            game_state.player_position.x,
            game_state.player_position.y,
            30.0,
            30.0,
        ),
        graphics::Color::WHITE,
    )?;
    graphics::draw(ctx, &player_rect, graphics::DrawParam::default())?;

    graphics::present(ctx)?;
    Ok(())
}

