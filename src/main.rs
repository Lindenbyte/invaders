use macroquad::prelude::*;

mod entity;
use entity::{
	player::Player,
	enemy::Enemy,
	projectile::{
		Projectile,
		PROJECTILE_DAMAGE,
		PROJECTILE_SIZE
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
	let mut enemies: Vec<Enemy> = vec![
		Enemy::new(
			vec2(screen_width() / 2.0, 0.0),
			100
		)
	];
	let mut projectiles: Vec<Projectile> = vec![];

	// Main loop
	while !is_key_pressed(KeyCode::Escape) {
		clear_background(BLACK);

		// Player Attacks
		if is_key_pressed(KeyCode::Up) {
			let projectile_offset = vec2(PROJECTILE_SIZE / 2.0, PROJECTILE_SIZE / 2.0);
			let position = *player.get_position() + *player.get_size() / 2.0 - projectile_offset;

			projectiles.push(
				Projectile::new(position)
			)
		}

		// for enemy in enemies.iter_mut() {
		// 	enemy.update();
		// 	enemy.render();
		// }

		enemies.retain_mut(|enemy| {
			enemy.update();
			enemy.render();

			return enemy.is_alive();
		});

		projectiles.retain_mut(|projectile| {
			projectile.update();
			projectile.render();

			for enemy in enemies.iter_mut() { 
				if projectile.get_rect().overlaps(&enemy.get_rect()) {
					enemy.damage(PROJECTILE_DAMAGE);
					projectile.hit();
				}
			}

			return projectile.is_alive();
		});

		player.update();
		player.render();

		// "Basic UI"
		draw_text(
			format!("HP: {}", player.get_life()).as_str(),
			10.0, screen_height() - player.get_size().y,
			18.0, WHITE
		);

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