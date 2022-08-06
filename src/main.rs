use core::panic;

use macroquad::prelude::*;

mod entity;
use entity::{
	Entity,
	EntityType,
	player::Player,
	enemy::Enemy
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
	entities.push(Box::new(Enemy::new(vec2(10.0, 0.0), 10)));

	// Main loop
	while !is_key_pressed(KeyCode::Escape) {
		clear_background(BLACK);

		for entity in entities.iter_mut() {
			entity.update();
			entity.render();

			match entity.get_type() {
				EntityType::Player => {},
				EntityType::Enemy => {},
				EntityType::Projectile => {},
			}
		}

		entities.retain(|entity| {
			match entity.get_type() {
				EntityType::Enemy => {
					if entity.get_position().y > screen_height()
					|| *entity.get_life() <= 0 {
						return false;
					}

					return true;
				},
				_ => return true
			}
		});

		next_frame().await;
	}
}