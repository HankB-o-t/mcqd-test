use macroquad::prelude::*;
use macroquad::texture;

const PLAYER_SIZE: Vec2 = const_vec2!([20.0, 20.0]);
const PLAYER_SPEED: f32 = 300.0;


struct Player {
    rect: Rect,
}

struct Enemy { 
    rect: Rect,
}

/*struct Bullet {
    circle: Circle,
} */

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

#[macroquad::main("Survive game")]
async fn main() {
    let mut cloc: f64  = 0.0;
    let mut dcloc: i32 = 0;

    let mut player = Player::new();

    let mut enemy1 = Enemy::new();
    let mut enemy2 = Enemy::new();
    let mut enemy3 = Enemy::new();
    
    let mut enemy4 = Enemy::new();
    
    let mut lives = 100;   

    let texture: Texture2D = load_texture("images/nave.png").await.unwrap();

    loop {
        player.update(get_frame_time());

        //IMPORTANTE:                    X    Y
        enemy1.update(get_frame_time(), 6.0, 12.0);
        enemy2.update(get_frame_time(), 12.0, 6.0);
        enemy3.update(get_frame_time(), cloc as f32 / 10.0, cloc as f32 / 10.0);
        enemy4.update(get_frame_time(), 6.0, 6.0);
        
        clear_background(BLACK);


        //player.draw();
        enemy1.draw(35.0, 35.0,BLUE);
        enemy2.draw(35.0, 35.0,GRAY);
        enemy3.draw(50.0, 50.0,RED);
        enemy4.draw(40.0, 40.0,GREEN);
        
        draw_texture(&texture, player.rect.x - 5.0, player.rect.y - 5.0, WHITE);
        
        draw_circle(screen_width() / 2.0, screen_height() / 2.0, 0.03, WHITE);

        if player.rect.intersect(enemy1.rect).is_some()
            || player.rect.intersect(enemy2.rect).is_some()
            || player.rect.intersect(enemy3.rect).is_some()
            || player.rect.intersect(enemy4.rect).is_some()
        {
            draw_rectangle(player.rect.x - 5.0, player.rect.y - 5.0, player.rect.w*2.0, player.rect.h*2.0, ORANGE);
            lives -= 1;
        }

        if lives == 0 || is_key_down(KeyCode::Q) {
            break;
        }
        draw_text(&format!("HP: {}", lives.to_string()).to_string(), 10.0, 20.0, 30.0, WHITE);
       
        // debug
        draw_text(&format!("Time: {}", dcloc.to_string()).to_string(), 10.0, screen_height() - 20.0, 30.0, WHITE);
       
        cloc = get_time();
        dcloc = cloc as i32;
        next_frame().await;
    }
}
