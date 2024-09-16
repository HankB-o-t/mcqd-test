use macroquad::prelude::*;
use macroquad::texture;
mod enemy;
mod player;

const PLAYER_SIZE: Vec2 = const_vec2!([20.0, 20.0]);
const PLAYER_SPEED: f32 = 300.0;

#[macroquad::main("Survive game")]
async fn main() {
    let mut cloc: f64  = 0.0;
    let mut dcloc: i32 = 0;
    let mut player = player::Player::new();
    
    let mut enemy1 = enemy::Enemy::new();
    let mut enemy2 = enemy::Enemy::new();
    let mut enemy3 = enemy::Enemy::new();
    let mut enemy4 = enemy::Enemy::new();
    
    let mut lives = 100;

    let texture: Texture2D = load_texture("images/nave.png").await.unwrap();

    loop {
        player.update(get_frame_time());

        //IMPORTANTE:                    X    Y
        enemy1.update(get_frame_time(), 5.1, 12.8);
        enemy2.update(get_frame_time(), 11.7, 6.2);
        enemy3.update(get_frame_time(), cloc as f32 / 3.14, cloc as f32 / 3.14);
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
        let mut fps = get_frame_time() * 1000.0;

        draw_text(&format!("Time: {}", dcloc.to_string()).to_string(), 10.0, screen_height() - 20.0, 30.0, WHITE);
        draw_text(&format!("FPS: {}", fps.to_string()).to_string(), 120.0, screen_height() - 20.0, 30.0, WHITE);
       
        cloc = get_time();
        dcloc = cloc as i32;
        next_frame().await;
    }
}
