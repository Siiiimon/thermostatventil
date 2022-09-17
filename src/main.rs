use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(700, 500)
        .title("Thermostatventil")
        .build();

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);
        d.draw_text("Hello, Ray!", 300, 235, 20, Color::LIGHTGRAY)
    }
}
