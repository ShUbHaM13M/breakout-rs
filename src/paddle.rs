use ggez::graphics::{Color, Mesh, Rect};
use ggez::input::keyboard::KeyCode;
use ggez::Context;
use ggez::{glam::Vec2, graphics, GameError};

pub const PADDLE_HEIGHT: f32 = 25.0;
pub const PADDLE_WIDTH: f32 = 160.0;
pub const PADDLE_HEIGHT_HALF: f32 = PADDLE_HEIGHT * 0.5;
pub const PADDLE_WIDTH_HALF: f32 = PADDLE_WIDTH * 0.5;

pub const PADDLE_RECT: Rect = Rect::new(
    -PADDLE_WIDTH_HALF,
    -PADDLE_HEIGHT_HALF,
    PADDLE_WIDTH,
    PADDLE_HEIGHT,
);

fn clamp(value: &mut f32, min: f32, max: f32) {
    if *value < min {
        *value = min;
    } else if *value > max {
        *value = max;
    }
}

pub struct Paddle {
    pub pos: Vec2,
    pub rect: Rect,
    pub color: Color,
    pub speed: f32,
}

impl Paddle {
    pub fn new(pos: Vec2, color: Color, speed: f32) -> Paddle {
        Paddle {
            pos,
            color,
            speed,
            rect: PADDLE_RECT,
        }
    }
    pub fn render(&self, ctx: &Context) -> Result<graphics::Mesh, GameError> {
        Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), self.rect, self.color)
    }
    pub fn move_player(&mut self, ctx: &Context, keycode: KeyCode, dir: f32) {
        let screen_width = ctx.gfx.drawable_size().0;
        let dt = ctx.time.delta().as_secs_f32();
        if ctx.keyboard.is_key_pressed(keycode) {
            self.pos.x += self.speed * dir * dt
        }

        clamp(
            &mut self.pos.x,
            PADDLE_WIDTH_HALF,
            screen_width - PADDLE_WIDTH_HALF,
        );
    }
}
