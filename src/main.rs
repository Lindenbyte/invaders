use macroquad::prelude::*;

mod entity;
use entity::{
	Entity,
	EntityType,
	player::Player,
	enemy::Enemy,
	projectile::{
		Projectile,
		PROJECTILE_DAMAGE, PROJECTILE_SIZE
	},
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
	let mut player: Player = Player::new();
	let mut entities: Vec<Box<dyn Entity>> = vec![];

	// Main loop
	while !is_key_pressed(KeyCode::Escape) {
		clear_background(BLACK);

		for entity in entities.iter_mut() {
			entity.update();
			entity.render();
			
			// TODO: Projectile collision
		}

		// Player Attacks
		if is_key_pressed(KeyCode::Up) {
			let projectile_offset = vec2(PROJECTILE_SIZE / 2.0, PROJECTILE_SIZE / 2.0);
			let position = *player.get_position() + *player.get_size() / 2.0 - projectile_offset;

			entities.push(Box::new(
				Projectile::new(position)
			))
		}

		entities.retain(|entity| {
			return entity.is_alive();
		});


		player.update();
		player.render();

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