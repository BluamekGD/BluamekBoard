use macroquad::prelude::*;
use std::fs;

// Window configuration
fn window_conf() -> Conf {
    Conf {
        window_title: "BluamekBoard :)".to_owned(),
        window_width: 400,
        window_height: 200,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // Read from board.conf
    let board_config = fs::read_to_string("board.conf").unwrap();

    for board_config_line in board_config.lines() {
        let board_config_line = board_config_line.trim();

        // Filter out comments
        if board_config_line.starts_with("//") || board_config_line.is_empty() {
            continue;
        }

        // Split by comma
        let parts: Vec<&str> = board_config_line.split(',').collect();

        // Clean each part
        let key = parts[0].trim();
        let x: f32 = parts[1].trim().parse().unwrap();
        let y: f32 = parts[2].trim().parse().unwrap();
        let w: f32 = parts[3].trim().parse().unwrap();
        let h: f32 = parts[4].trim().parse().unwrap();
        let label = parts[5].trim().trim_matches('"');

        // Print result
        println!("key: {}", key);
        println!("x: {}", x);
        println!("y: {}", y);
        println!("w: {}", w);
        println!("h: {}", h);
        println!("label: {}", label);
    }
}
