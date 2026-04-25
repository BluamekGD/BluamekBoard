use macroquad::prelude::*;

// Window configuration
fn window_conf() -> Conf {
    Conf {
        window_title: "BetterBoard :)".to_owned(),
        window_width: 400,
        window_height: 200,
        ..Default::default()
    }
}

// Create window
#[macroquad::main(window_conf)]

// Draw W square
async fn main() {
    loop{
        draw_rectangle(screen_width() / 2.0 - 90.0, 10.0, 60.0, 60.0, DARKGRAY);
        if is_key_down(KeyCode::W) {
        draw_rectangle(screen_width() / 2.0 - 90.0, 10.0, 60.0, 60.0, LIGHTGRAY);
        }
        next_frame().await
    }
}
