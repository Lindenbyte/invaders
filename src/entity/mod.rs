
use macroquad::prelude::*;

pub mod player;
pub mod enemy;

pub enum EntityType {
	Player,
	Enemy,
	Projectile,
}

pub trait Entity {
	fn update(&mut self);
	fn render(&self);

	fn get_type(&self) -> &EntityType;
	fn get_life(&self) -> &i32;
	fn get_position(&self) -> &Vec2;
	fn get_size(&self) -> &Vec2;
}