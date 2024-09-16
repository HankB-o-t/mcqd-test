use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = const_vec2!([20.0, 20.0]);
const PLAYER_SPEED: f32 = 300.0;

pub struct Player {
    pub rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() / 2.0,
                screen_height() / 2.0,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, GREEN);
    }
    
    pub fn update(&mut self, dt: f32) {


        // movimiento
        let mut x_move: f32 = 0.0;
        let mut y_move: f32 = 0.0;

        if is_key_down(KeyCode::D) { x_move += 1.0 }
        if is_key_down(KeyCode::A) { x_move -= 1.0 }
        if is_key_down(KeyCode::W) { y_move -= 1.0 }
        if is_key_down(KeyCode::S) { y_move += 1.0 }

        self.rect.x += x_move * dt * PLAYER_SPEED;
        self.rect.y += y_move * dt * PLAYER_SPEED;
    }
}

