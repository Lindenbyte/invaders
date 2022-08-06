use macroquad::prelude::*;

pub const PROJECTILE_SIZE: f32				= 10.0;
const PROJECTILE_VELOCITY: f32 				= 5.0;
pub const PROJECTILE_DEFAULT_DAMAGE: i32 	= 10;

#[derive(PartialEq, Clone, Copy)]
pub struct ProjectileID(i32);
static mut PROJECTILE_COUNT: i32 = 0;

pub struct Projectile {
	id: ProjectileID,
	position: Vec2,
	size: Vec2,
	// angle: Vec2,
	life: i32,
	damage: i32,
}

impl Projectile {
	pub fn new(position: Vec2, damage: i32) -> Self {
		unsafe {
			let id = ProjectileID(PROJECTILE_COUNT);
			PROJECTILE_COUNT += 1;
			
			return Self {
				id,
				position,
				size: vec2(PROJECTILE_SIZE, PROJECTILE_SIZE),
				// angle: vec2(0.0, 0.0),
				life: 2,
				damage: damage
			}
		}
	}

	pub fn update(&mut self) {
		// TODO: Calculate new position based on velocity and angle
		self.position.y -= PROJECTILE_VELOCITY;
	}

	pub fn render(&self) {
		draw_rectangle(self.position.x, self.position.y, self.size.x, self.size.y, WHITE);
	}


	pub fn get_id(&self) -> &ProjectileID {
		return &self.id;
	}

	pub fn get_damage(&self) -> i32 {
		return self.damage;
	} 


	pub fn get_rect(&self) -> Rect {
		return Rect { 
			x: self.position.x, 
			y: self.position.y, 
			w: self.size.x,
			h: self.size.y
		}
	}

	pub fn hit(&mut self) {
		self.life -= 1;
	}


	pub fn is_alive(&self) -> bool {
		if self.position.x <= 0.0 - self.size.x
		|| self.position.x >= screen_width()
		|| self.position.y <= 0.0 - self.size.y
		|| self.position.y >= screen_height() 
		|| self.life < 1 {			
			return false;
		}

		return true;
	}
}