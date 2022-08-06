use macroquad::prelude::*;

use super::{
	Entity,
	EntityType
};

const PLAYER_WIDTH: f32 = 25.0;
const PLAYER_HEIGHT: f32 = 25.0;
const PLAYER_VELOCITY: f32 = 3.0;

pub struct Player {
	position: Vec2,
	size: Vec2,
	entity_type: EntityType
}

impl Player {
	pub fn new() -> Self {
		return Self {
			position: vec2(
				(screen_width() / 2.0) - (PLAYER_WIDTH / 2.0),
				screen_height() - (PLAYER_HEIGHT * 2.0)
				),
			size: vec2(PLAYER_WIDTH, PLAYER_HEIGHT),
			entity_type: EntityType::Player
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
}