use macroquad::prelude::*;

use super::projectile::ProjectileID;

const ENEMY_DEFAULT_SIZE: f32 		= 30.0;
const ENEMY_LIFE_TO_SIZE_RATIO: f32 = 0.2;
const ENEMY_VELOCITY: f32 			= 0.25;
pub const ENEMY_SPAWN_CHANCE: f32 	= 1.0 / 300.0;

pub struct Enemy {
	position: Vec2,
	size: Vec2,
	life: i32,
	damaged_by: Vec<ProjectileID>,
}

impl Enemy {
	pub fn new(position: Vec2, life: i32, size: Vec2) -> Self {
		return Self {
			position,
			size,
			life,
			damaged_by: vec![]
		}
	}

	pub fn calculate_size(life: i32) -> Vec2 {
		return vec2(ENEMY_DEFAULT_SIZE, ENEMY_DEFAULT_SIZE)
			* (life as f32 * ENEMY_LIFE_TO_SIZE_RATIO).clamp(0.5, 5.0)
	}


	pub fn update(&mut self) {
		self.position.y += ENEMY_VELOCITY;
	}

	pub fn render(&self) {
		draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, RED);
	}


	pub fn damage(&mut self, hit_by: &ProjectileID, amount: i32) {
		self.damaged_by.push(*hit_by);
		self.life -= amount;
		
		let prev_size = self.size;
		self.size = Enemy::calculate_size(self.life);

		let size_diff = prev_size - self.size;
		self.position += size_diff / 2.0;
	}

	pub fn damaged_by(&self, projectile: &ProjectileID) -> bool {
		for previously in self.damaged_by.iter() {
			if *previously == *projectile {
				return true;
			}
		}

		return false;
	}


	pub fn get_rect(&self) -> Rect {
		return Rect { 
			x: self.position.x, 
			y: self.position.y, 
			w: self.size.x,
			h: self.size.y
		}
	}
	
	pub fn is_alive(&self) -> bool {
		if self.position.y > screen_height()
		|| self.life <= 0 {
			return false;
		}
		
		return true;
	}
}
