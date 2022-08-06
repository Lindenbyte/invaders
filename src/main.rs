use macroquad::prelude::*;

fn window_config() -> Conf {
	return Conf { 
		window_title: "Invaders".to_owned(),
		fullscreen: true, 
		window_resizable: false,
		..Default::default()
	}
}

#[macroquad::main(window_config)]
async fn main() {

	while !is_key_pressed(KeyCode::Escape) {
		clear_background(BLACK);

		draw_line(0.0, 0.0, screen_width(), screen_height(), 2.0, WHITE);

		next_frame().await;
	}
}