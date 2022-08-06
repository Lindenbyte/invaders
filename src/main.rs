use macroquad::prelude::*;

mod entity;
use entity::{
	player::Player,
	enemy::{
		Enemy,
		ENEMY_SPAWN_CHANCE
	},
	projectile::{
		Projectile,
		PROJECTILE_DEFAULT_DAMAGE,
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
	let mut kills: i32 = 0;

	let mut enemies: Vec<Enemy> = vec![];	
	let mut projectiles: Vec<Projectile> = vec![];

	// Main loop
	while !is_key_pressed(KeyCode::Escape) {
		clear_background(BLACK);

		// Player Attacks
		if is_key_pressed(KeyCode::Up) {
			let projectile_offset = vec2(PROJECTILE_SIZE / 2.0, PROJECTILE_SIZE / 2.0);
			let position = *player.get_position() + *player.get_size() / 2.0 - projectile_offset;

			projectiles.push(
				Projectile::new(position, PROJECTILE_DEFAULT_DAMAGE)
			)
		}

		let enemy_spawn_roll: f32 = rand::gen_range(0.0, 1.0);
		if enemy_spawn_roll <= ENEMY_SPAWN_CHANCE {
			let life: i32 = rand::gen_range(10, 60); 
			let size = Enemy::calculate_size(life);
			let position = vec2(
				rand::gen_range(20.0, screen_width() - size.x),
				0.0
			);
			
			enemies.push(Enemy::new(position, life, size));
		}

		enemies.retain_mut(|enemy| {
			enemy.update();
			enemy.render();

			return enemy.is_alive();
		});

		projectiles.retain_mut(|projectile| {
			projectile.update();
			projectile.render();

			for enemy in enemies.iter_mut() {
				let id = projectile.get_id();
				
				if projectile.get_rect().overlaps(&enemy.get_rect()) && !enemy.damaged_by(id) {
					enemy.damage(id, projectile.get_damage());
					if !enemy.is_alive() {
						kills += 1;
					}
					
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
			24.0, WHITE
		);

		draw_text(
			format!("Kills: {}", kills).as_str(),
			100.0, screen_height() - player.get_size().y,
			24.0, WHITE
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