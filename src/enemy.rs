use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = const_vec2!([20.0, 20.0]);
const PLAYER_SPEED: f32 = 300.0;

pub struct Enemy { 
    pub rect: Rect,
}

impl Enemy {
    pub fn new() -> Self {
        Self {
            rect: Rect::new( 
                screen_width() / 2.5,
                screen_height() / 2.5,
                PLAYER_SIZE.x * 2.0,
                PLAYER_SIZE.y * 2.0,
            ),
        }
    }

    pub fn update(&mut self, shit: f32, shit_x: f32, shit_y: f32) {
        if self.rect.x >= screen_width() {
            self.rect.x = 0.0;
        } self.rect.x += shit_x;

        if self.rect.y >= screen_height() {
            self.rect.y = 0.0;
        } self.rect.y += shit_y;
    }

    pub fn draw(&self, width: f32, height: f32,color: Color) {
        draw_rectangle(self.rect.x, self.rect.y, width, height, color);
    }
}
