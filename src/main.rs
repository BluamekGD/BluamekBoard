use macroquad::prelude::*;

#[macroquad::main("BetterBoard :)")]

async fn main() {
    loop{
        draw_rectangle(screen_width() / 2.0 - 90.0, 10.0, 60.0, 60.0, DARKGRAY);
        if is_key_down(KeyCode::W) {
        draw_rectangle(screen_width() / 2.0 - 90.0, 10.0, 60.0, 60.0, LIGHTGRAY);
        }
        next_frame().await
    }
}
