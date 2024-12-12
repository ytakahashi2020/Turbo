turbo::cfg! {r#"
    name = "Game name"
    version = "1.0.0"
    author = "game author"
    description = "Game description"
    [settings]
    resolution = [256, 144]
"#}

turbo::init! {
    // Define the GameState struct.
    struct GameState {
        x_speed: f32,
        y_speed: f32,
        moving: bool
    } = {
        // Set the struct's initial value.
        Self {
            x_speed: 0.0,
            y_speed: 0.0,
            moving: false
        }
    }
}

turbo::go! {
    let mut state = GameState::load();

    clear!(0x5511ffff);
    circ!(d = 16, x = (state.x_speed as i32 ) +120, y = (state.y_speed as i32 ) + 64, color = 0x011000ff);

    if gamepad(0).up.pressed() {
        state.y_speed += - 1.0;
        state.moving = true;
    }
    if gamepad(0).down.pressed() {
        state.y_speed += 1.0;
        state.moving = true;
    }
    if gamepad(0).right.pressed() {
        state.x_speed += 1.0;
        state.moving = true;
    }
    if gamepad(0).left.pressed() {
        state.x_speed += - 1.0;
        state.moving = true;
    }


    state.save();
}