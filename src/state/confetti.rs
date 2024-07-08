// source: https://github.com/Handfish/confetty_rs

use ratatui::style::Color;

use crate::consts::FRAME_RATE_SECS;

#[derive(Clone)]
pub struct Particle {
	pub char: char,
	pub color: Color,
	x: f32,
	y: f32,
	x_velocity: f32,
	y_velocity: f32,
}

impl Particle {
	fn new(x: u16, y: u16) -> Self {
		Particle {
			char: random_character(),
			color: random_color(),
			x: x as f32,
			y: y as f32,
			x_velocity: -20.0 + fastrand::f32() * 40.0,
			y_velocity: -5.0 + fastrand::f32() * 10.0,
		}
	}

	fn update(&mut self) {
		self.x += self.x_velocity * FRAME_RATE_SECS;
		self.y += self.y_velocity * FRAME_RATE_SECS;
		self.y_velocity += 9.8 * FRAME_RATE_SECS;
	}

	pub fn pos(&self) -> (u16, u16) {
		(self.x.floor() as u16, self.y.floor() as u16)
	}
}

fn random_character() -> char {
	const CHARACTERS: [char; 6] = ['█', '▓', '▒', '░', '▄', '▀'];
	fastrand::choice(CHARACTERS).unwrap_or(CHARACTERS[0])
}

fn random_color() -> Color {
	const COLORS: [Color; 5] = [
		Color::Rgb(168, 100, 253), // #a864fd
		Color::Rgb(41, 205, 255),  // #29cdff
		Color::Rgb(120, 255, 68),  // #78ff44
		Color::Rgb(255, 113, 141), // #ff718d
		Color::Rgb(253, 255, 106), // #fdff6a
	];
	fastrand::choice(COLORS).unwrap_or(COLORS[0])
}

#[derive(Clone, Default)]
pub struct ConfettiState {
	points: Vec<(usize, usize)>,
	pub particles: Vec<Particle>,
}

impl ConfettiState {
	pub fn push_points(&mut self, x: usize, y: usize) {
		self.points.push((x, y));
	}

	pub fn is_target_point(&mut self, x: usize, y: usize) -> bool {
		let idx = self.points.iter().position(|p| p.0 == x && p.1 == y);
		let ret = idx.is_some();
		if let Some(i) = idx {
			self.points.swap_remove(i);
		}
		ret
	}

	pub fn update_particles(&mut self) {
		for p in self.particles.iter_mut() {
			p.update();
		}
	}

	pub fn spawn_particles(&mut self, x: u16, y: u16, width: u16, height: u16) {
		let x = x + width / 2;
		let y = y + height / 2;
		for _ in 0..2 {
			self.particles.push(Particle::new(x, y));
		}
	}

	pub fn remove_particles(&mut self, idxs: Vec<usize>) {
		for i in idxs.into_iter().rev() {
			self.particles.swap_remove(i);
		}
	}

	pub fn reset(&mut self) {
		self.points.clear();
		self.particles.clear();
	}
}
