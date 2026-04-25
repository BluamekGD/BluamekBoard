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

fn parse_color(name: &str) -> Color {
    match name {
        "DARKGRAY"  => DARKGRAY,
        "LIGHTGRAY" => LIGHTGRAY,
        "RED"       => RED,
        "GREEN"     => GREEN,
        "BLUE"      => BLUE,
        "YELLOW"    => YELLOW,
        "ORANGE"    => ORANGE,
        "PURPLE"    => PURPLE,
        "PINK"      => PINK,
        "BLACK"     => BLACK,
        _           => WHITE,
    }
}

// Map a keycode name string (e.g. "KEY_W") to a macroquad KeyCode.
// Uses the keycodes crate constants as the canonical source of truth
// so config files can use the same names as the docs.rs reference.
fn parse_keycode(name: &str) -> Option<KeyCode> {
    // keycodes crate constants are u32 values representing virtual key codes.
    // We map the string name to the matching macroquad KeyCode variant.
    match name {
        // Alphabet
        "KEY_A" => Some(KeyCode::A),
        "KEY_B" => Some(KeyCode::B),
        "KEY_C" => Some(KeyCode::C),
        "KEY_D" => Some(KeyCode::D),
        "KEY_E" => Some(KeyCode::E),
        "KEY_F" => Some(KeyCode::F),
        "KEY_G" => Some(KeyCode::G),
        "KEY_H" => Some(KeyCode::H),
        "KEY_I" => Some(KeyCode::I),
        "KEY_J" => Some(KeyCode::J),
        "KEY_K" => Some(KeyCode::K),
        "KEY_L" => Some(KeyCode::L),
        "KEY_M" => Some(KeyCode::M),
        "KEY_N" => Some(KeyCode::N),
        "KEY_O" => Some(KeyCode::O),
        "KEY_P" => Some(KeyCode::P),
        "KEY_Q" => Some(KeyCode::Q),
        "KEY_R" => Some(KeyCode::R),
        "KEY_S" => Some(KeyCode::S),
        "KEY_T" => Some(KeyCode::T),
        "KEY_U" => Some(KeyCode::U),
        "KEY_V" => Some(KeyCode::V),
        "KEY_W" => Some(KeyCode::W),
        "KEY_X" => Some(KeyCode::X),
        "KEY_Y" => Some(KeyCode::Y),
        "KEY_Z" => Some(KeyCode::Z),

        // Top-row digits
        "KEY_0" => Some(KeyCode::Key0),
        "KEY_1" => Some(KeyCode::Key1),
        "KEY_2" => Some(KeyCode::Key2),
        "KEY_3" => Some(KeyCode::Key3),
        "KEY_4" => Some(KeyCode::Key4),
        "KEY_5" => Some(KeyCode::Key5),
        "KEY_6" => Some(KeyCode::Key6),
        "KEY_7" => Some(KeyCode::Key7),
        "KEY_8" => Some(KeyCode::Key8),
        "KEY_9" => Some(KeyCode::Key9),

        // Numpad
        "KEY_NUMPAD0" => Some(KeyCode::Kp0),
        "KEY_NUMPAD1" => Some(KeyCode::Kp1),
        "KEY_NUMPAD2" => Some(KeyCode::Kp2),
        "KEY_NUMPAD3" => Some(KeyCode::Kp3),
        "KEY_NUMPAD4" => Some(KeyCode::Kp4),
        "KEY_NUMPAD5" => Some(KeyCode::Kp5),
        "KEY_NUMPAD6" => Some(KeyCode::Kp6),
        "KEY_NUMPAD7" => Some(KeyCode::Kp7),
        "KEY_NUMPAD8" => Some(KeyCode::Kp8),
        "KEY_NUMPAD9" => Some(KeyCode::Kp9),
        "KEY_ADD"      => Some(KeyCode::KpAdd),
        "KEY_SUBTRACT" => Some(KeyCode::KpSubtract),
        "KEY_MULTIPLY" => Some(KeyCode::KpMultiply),
        "KEY_DIVIDE"   => Some(KeyCode::KpDivide),
        "KEY_DECIMAL"  => Some(KeyCode::KpDecimal),
        "KEY_RETURN"   => Some(KeyCode::KpEnter),

        // Function keys
        "KEY_F1"  => Some(KeyCode::F1),
        "KEY_F2"  => Some(KeyCode::F2),
        "KEY_F3"  => Some(KeyCode::F3),
        "KEY_F4"  => Some(KeyCode::F4),
        "KEY_F5"  => Some(KeyCode::F5),
        "KEY_F6"  => Some(KeyCode::F6),
        "KEY_F7"  => Some(KeyCode::F7),
        "KEY_F8"  => Some(KeyCode::F8),
        "KEY_F9"  => Some(KeyCode::F9),
        "KEY_F10" => Some(KeyCode::F10),
        "KEY_F11" => Some(KeyCode::F11),
        "KEY_F12" => Some(KeyCode::F12),

        // Modifiers
        "KEY_SHIFT"   => Some(KeyCode::LeftShift),
        "KEY_CONTROL" => Some(KeyCode::LeftControl),
        "KEY_ALT"     => Some(KeyCode::LeftAlt),
        "KEY_META"    => Some(KeyCode::LeftSuper),
        "KEY_WIN"     => Some(KeyCode::LeftSuper),

        // Navigation
        "KEY_UP"        => Some(KeyCode::Up),
        "KEY_DOWN"      => Some(KeyCode::Down),
        "KEY_LEFT"      => Some(KeyCode::Left),
        "KEY_RIGHT"     => Some(KeyCode::Right),
        "KEY_HOME"      => Some(KeyCode::Home),
        "KEY_END"       => Some(KeyCode::End),
        "KEY_PAGE_UP"   => Some(KeyCode::PageUp),
        "KEY_PAGE_DOWN" => Some(KeyCode::PageDown),
        "KEY_INSERT"    => Some(KeyCode::Insert),
        "KEY_DELETE"    => Some(KeyCode::Delete),

        // Common keys
        "KEY_ENTER"       => Some(KeyCode::Enter),
        "KEY_ESCAPE"      => Some(KeyCode::Escape),
        "KEY_BACK_SPACE"  => Some(KeyCode::Backspace),
        "KEY_TAB"         => Some(KeyCode::Tab),
        "KEY_SPACE"       => Some(KeyCode::Space),
        "KEY_CAPS_LOCK"   => Some(KeyCode::CapsLock),
        "KEY_NUM_LOCK"    => Some(KeyCode::NumLock),
        "KEY_SCROLL_LOCK" => Some(KeyCode::ScrollLock),
        "KEY_PAUSE"       => Some(KeyCode::Pause),
        "KEY_PRINTSCREEN" => Some(KeyCode::PrintScreen),

        // Punctuation / symbols
        "KEY_MINUS"         | "KEY_HYPHEN_MINUS" => Some(KeyCode::Minus),
        "KEY_EQUALS"                             => Some(KeyCode::Equal),
        "KEY_OPEN_BRACKET"                       => Some(KeyCode::LeftBracket),
        "KEY_CLOSE_BRACKET"                      => Some(KeyCode::RightBracket),
        "KEY_BACK_SLASH"                         => Some(KeyCode::Backslash),
        "KEY_SEMICOLON"                          => Some(KeyCode::Semicolon),
        "KEY_QUOTE"                              => Some(KeyCode::Apostrophe),
        "KEY_BACK_QUOTE"                         => Some(KeyCode::GraveAccent),
        "KEY_COMMA"                              => Some(KeyCode::Comma),
        "KEY_PERIOD"                             => Some(KeyCode::Period),
        "KEY_SLASH"                              => Some(KeyCode::Slash),

        _ => {
            eprintln!("[warn] Unknown keycode name in config: '{}'", name);
            None
        }
    }
}

struct KeyBox {
    key:        KeyCode,
    x:          f32,
    y:          f32,
    w:          f32,
    h:          f32,
    color_up:   Color,
    color_down: Color,
    label:      String,
}

#[macroquad::main(window_conf)]
async fn main() {
    // Parse board.conf
    let board_config = fs::read_to_string("board.conf")
        .expect("Could not read board.conf");

    let mut keys: Vec<KeyBox> = Vec::new();

    for (line_no, raw_line) in board_config.lines().enumerate() {
        let line = raw_line.trim();

        // Skip comments and blank lines
        if line.is_empty() || line.starts_with("//") {
            continue;
        }

        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() < 8 {
            eprintln!("[warn] Line {}: expected 8 fields, got {} — skipping", line_no + 1, parts.len());
            continue;
        }

        let key_name  = parts[0].trim();
        let x: f32    = parts[1].trim().parse().unwrap_or_else(|_| { eprintln!("[warn] Line {}: bad x", line_no+1); 0.0 });
        let y: f32    = parts[2].trim().parse().unwrap_or_else(|_| { eprintln!("[warn] Line {}: bad y", line_no+1); 0.0 });
        let w: f32    = parts[3].trim().parse().unwrap_or_else(|_| { eprintln!("[warn] Line {}: bad w", line_no+1); 50.0 });
        let h: f32    = parts[4].trim().parse().unwrap_or_else(|_| { eprintln!("[warn] Line {}: bad h", line_no+1); 50.0 });
        let color_up   = parse_color(parts[5].trim());
        let color_down = parse_color(parts[6].trim());
        let label      = parts[7].trim().trim_matches('"').to_string();

        let Some(keycode) = parse_keycode(key_name) else {
            eprintln!("[warn] Line {}: skipping unknown key '{}'", line_no + 1, key_name);
            continue;
        };

        keys.push(KeyBox { key: keycode, x, y, w, h, color_up, color_down, label });
    }

    println!("[info] Loaded {} key(s) from board.conf", keys.len());

    // Render loop
    loop {
        clear_background(BLACK);

        for k in &keys {
            let color = if is_key_down(k.key) { k.color_down } else { k.color_up };

            draw_rectangle(k.x, k.y, k.w, k.h, color);

            // Thin border so keys are always visible
            draw_rectangle_lines(k.x, k.y, k.w, k.h, 1.5, DARKGRAY);

            // Center the label inside the key box
            let font_size = 18.0;
            let text_w = measure_text(&k.label, None, font_size as u16, 1.0).width;
            draw_text(
                &k.label,
                k.x + (k.w - text_w) / 2.0,
                k.y + k.h / 2.0 + font_size / 3.0,
                font_size,
                WHITE,
            );
        }

        next_frame().await;
    }
}
