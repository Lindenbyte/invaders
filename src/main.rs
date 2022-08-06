use macroquad::prelude::*;

mod entity;
use entity::{
	Entity,
	player::Player
};

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
	// Setup
	let mut entities: Vec<Box<dyn Entity>> = vec![];
	entities.push(Box::new(Player::new()));

	// Main loop
	while !is_key_pressed(KeyCode::Escape) {
		clear_background(BLACK);

		for entity in entities.iter_mut() {
			entity.update();
			entity.render();
		}

		next_frame().await;
	}
}