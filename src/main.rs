use macroquad::prelude::*;

mod entity;
use entity::{
	Entity,
	EntityType,
	player::Player,
	enemy::Enemy,
	projectile::Projectile,
};

// TODO: Move window back to fullscreen later, changed to smaller size for stream.
fn window_config() -> Conf {
	return Conf { 
		window_title: "Invaders".to_owned(),
		window_width: 1366,
		window_height: 768,
		// fullscreen: true, 
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
	entities.push(Box::new(Projectile::new(vec2(100.0, 100.0))));

	// Main loop
	while !is_key_pressed(KeyCode::Escape) {
		clear_background(BLACK);

		for entity in entities.iter_mut() {
			entity.update();
			entity.render();

			// match entity.get_type() {
			// 	EntityType::Player => {},
			// 	EntityType::Enemy => {},
			// 	EntityType::Projectile => {},
			// }
		}

		entities.retain(|entity| {
			return entity.should_be_kept();
		});

		// Debug information to screen
		#[cfg(debug_assertions)] {
			draw_text(
				format!("FPS: {}", get_fps()).as_str(), 
				10.0, 28.0, 
				18.0, GREEN
			);
		}

		next_frame().await;
	}
}