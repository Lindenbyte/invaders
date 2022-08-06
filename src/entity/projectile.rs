use macroquad::prelude::*;

use super::{
	Entity,
	EntityType
};

pub const PROJECTILE_SIZE: f32		= 10.0;
const PROJECTILE_VELOCITY: f32 		= 5.0;
pub const PROJECTILE_DAMAGE: i32 	= 10;

pub struct Projectile {
	entity_type: EntityType,
	position: Vec2,
	size: Vec2,
	life: i32,
	angle: Vec2,
}

impl Projectile {
	pub fn new(position: Vec2) -> Self {
		return Self {
			entity_type: EntityType::Projectile,
			position,
			size: vec2(PROJECTILE_SIZE, PROJECTILE_SIZE),
			angle: vec2(0.0, 0.0),
			life: 1
		}
	}
}

impl Entity for Projectile {
	fn update(&mut self) {
		// TODO: Calculate new position based on velocity and angle
		self.position.y -= PROJECTILE_VELOCITY;
	}

	fn render(&self) {
		draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, WHITE);
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

	fn get_life(&self) -> i32 {
		return self.life;
	}


	fn is_alive(&self) -> bool {
		if self.position.x <= 0.0 - self.size.x
		|| self.position.x >= screen_width()
		|| self.position.y <= 0.0 - self.size.y
		|| self.position.y >= screen_height() {			
			return false;
		}

		return true;
	}
}