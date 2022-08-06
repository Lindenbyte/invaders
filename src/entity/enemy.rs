use macroquad::prelude::*;

use super::{
	Entity,
	EntityType
};

const ENEMY_DEFAULT_WIDTH: f32 		= 50.0;
const ENEMY_DEFAULT_HEIGHT: f32 	= 50.0;
const ENEMY_LIFE_TO_SIZE_RATIO: f32 = 0.1;
const ENEMY_VELOCITY: f32 			= 1.0;

pub struct Enemy {
	entity_type: EntityType,
	position: Vec2,
	size: Vec2,
	life: i32,
}

impl Enemy {
	pub fn new(position: Vec2, life: i32) -> Self {
		return Self {
			entity_type: EntityType::Enemy,
			position,
			size: vec2(
				ENEMY_DEFAULT_WIDTH, 
				ENEMY_DEFAULT_HEIGHT) * (life as f32 * ENEMY_LIFE_TO_SIZE_RATIO
				),
			life,
		}
	}
}

impl Entity for Enemy {
	fn update(&mut self) {
		self.size = vec2(
			ENEMY_DEFAULT_WIDTH, 
			ENEMY_DEFAULT_HEIGHT) * (self.life as f32 * ENEMY_LIFE_TO_SIZE_RATIO
			);
		self.position.y += ENEMY_VELOCITY;
	}

	fn render(&self) {
		draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, RED);
	}

	fn get_type(&self) -> &EntityType {
		return &self.entity_type;
	}
	
	fn get_position(&self) -> &Vec2 {
		return &self.position;
	}
	
	fn get_size(&self) -> &Vec2 {
		return &self.size;
	}

	fn get_life(&self) -> &i32 {
		return &self.life;
	}
	
	fn is_alive(&self) -> bool {
		if self.position.y > screen_height()
		|| self.life <= 0 {
			println!("Enemy removed!");
			return false;
		}
		
		return true;
	}
}