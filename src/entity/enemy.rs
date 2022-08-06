use macroquad::prelude::*;

const ENEMY_DEFAULT_WIDTH: f32 		= 50.0;
const ENEMY_DEFAULT_HEIGHT: f32 	= 50.0;
const ENEMY_LIFE_TO_SIZE_RATIO: f32 = 0.1;
const ENEMY_VELOCITY: f32 			= 1.0;

pub struct Enemy {
	position: Vec2,
	size: Vec2,
	life: i32,
}

impl Enemy {
	pub fn new(position: Vec2, life: i32) -> Self {
		return Self {
			position,
			size: vec2(ENEMY_DEFAULT_WIDTH, ENEMY_DEFAULT_HEIGHT) 
				* (life as f32 * ENEMY_LIFE_TO_SIZE_RATIO).clamp(0.5, 5.0),
			life,
		}
	}

	pub fn update(&mut self) {
		self.position.y += ENEMY_VELOCITY;
	}

	pub fn render(&self) {
		draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, RED);
	}
	
	pub fn get_position(&self) -> &Vec2 {
		return &self.position;
	}

	pub fn damage(&mut self, amount: i32) {
		self.life -= amount;
		
		let prev_size = self.size;
		self.size = vec2(ENEMY_DEFAULT_WIDTH, ENEMY_DEFAULT_HEIGHT) 
			* (self.life as f32 * ENEMY_LIFE_TO_SIZE_RATIO).clamp(0.5, 5.0);

		let size_diff = prev_size - self.size;
		self.position += size_diff / 2.0;
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
			println!("Enemy removed!");
			return false;
		}
		
		return true;
	}
}
