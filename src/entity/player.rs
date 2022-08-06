use macroquad::prelude::*;

use super::{
	Entity,
	EntityType
};

const PLAYER_WIDTH: f32 	= 40.0;
const PLAYER_HEIGHT: f32 	= 40.0;
const PLAYER_VELOCITY: f32 	= 3.0;

pub struct Player {
	entity_type: EntityType,
	position: Vec2,
	size: Vec2,
	life: i32,
}

impl Player {
	pub fn new() -> Self {
		return Self {
			entity_type: EntityType::Player,
			position: vec2(
				(screen_width() / 2.0) - (PLAYER_WIDTH / 2.0),
				screen_height() - (PLAYER_HEIGHT * 2.0)
			),
			size: vec2(PLAYER_WIDTH, PLAYER_HEIGHT),
			life: 100,
		}
	}
}

impl Entity for Player {
	fn update(&mut self) {
		if is_key_down(KeyCode::A) {
			self.position.x -= PLAYER_VELOCITY;
		}

		if is_key_down(KeyCode::D) {
			self.position.x += PLAYER_VELOCITY;
		}

		if is_key_down(KeyCode::W) {
			self.position.y -= PLAYER_VELOCITY;
		}

		if is_key_down(KeyCode::S) {
			self.position.y += PLAYER_VELOCITY;
		}

		self.position = self.position.clamp(
			vec2(0.0, screen_height() / 3.0),
			vec2(screen_width() - self.size.x, screen_height() - (PLAYER_HEIGHT * 3.0))
		);
	}

	fn render(&self) {
		draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, BLUE);
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
		if self.life <= 0 {
			println!("Player removed!");
			return false;
		}

		return true;
	}
}