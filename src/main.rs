use ggez::event;
use ggez::glam::*;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::input::keyboard::KeyCode;
use ggez::{Context, GameResult};

mod paddle;
use paddle::{Paddle, PADDLE_HEIGHT_HALF};

struct MainState {
    paused: bool,
    player: Paddle,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let (screen_width, screen_height) = ctx.gfx.drawable_size();
        let state = MainState {
            paused: true,
            player: Paddle::new(
                Vec2::new(screen_width * 0.5, screen_height * 0.9 - PADDLE_HEIGHT_HALF),
                graphics::Color::WHITE,
                80.0,
            ),
        };
        Ok(state)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if self.paused {
            if ctx.keyboard.is_key_pressed(KeyCode::Space) {
                self.paused = !self.paused;
                return Ok(());
            }
        }

        self.player
            .move_player(ctx, KeyCode::A, -1.0);
        self.player
            .move_player(ctx, KeyCode::D, 1.0);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        let player_rect = self.player.render(&ctx)?;

        canvas.draw(&player_rect, self.player.pos);

        canvas.finish(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Breakout", "Shubham");
    let (mut ctx, event_loop) = cb.build()?;
    let state = MainState::new(&mut ctx)?;
    ctx.gfx.set_window_title("Breakout");
    event::run(ctx, event_loop, state)
}
