// source: https://github.com/Handfish/confetty_rs

use std::slice::Iter;

use ratatui::style::Color;

use crate::{common::Reset, consts::FRAME_RATE_SECS, global::global_setting};

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
			x_velocity: random_f32(10.0, 15.0),
			y_velocity: random_f32(10.0, 15.0),
		}
	}

	fn update(&mut self) {
		self.x += self.x_velocity * FRAME_RATE_SECS;
		self.y += self.y_velocity * FRAME_RATE_SECS;
		if self.x_velocity > 0.0 {
			self.x_velocity -= 1.0;
		}
		self.y_velocity += 9.8 * FRAME_RATE_SECS;
	}

	pub fn pos(&self) -> (u16, u16) {
		let x = self.x.floor() as u16;
		let y = self.y.floor() as u16;

		(x, y)
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

fn random_f32(min: f32, max: f32) -> f32 {
	fastrand::f32() * (max - min) + min
}

#[derive(Clone, Default)]
pub struct Particles {
	value: Vec<Particle>,
	points: Vec<(usize, usize)>,
}

impl Particles {
	pub fn push_point(&mut self, x: usize, y: usize) {
		if !global_setting().particle() {
			return;
		}
		self.points.push((x, y));
	}

	pub fn check_and_remove_point(&mut self, x: usize, y: usize) -> bool {
		let idx = self.points.iter().position(|p| p.0 == x && p.1 == y);
		let ret = idx.is_some();
		if let Some(i) = idx {
			self.points.swap_remove(i);
		}
		ret
	}

	pub fn update(&mut self) {
		for p in &mut self.value {
			p.update();
		}
	}

	pub fn remove(&mut self, idxs: Vec<usize>) {
		for i in idxs.into_iter().rev() {
			self.value.swap_remove(i);
		}
	}

	pub fn spawn(&mut self, x: u16, y: u16) {
		for _ in 0..5 {
			self.value.push(Particle::new(x, y));
		}
	}

	pub fn iter(&self) -> Iter<Particle> {
		self.value.iter()
	}
}

impl Reset for Particles {
	fn reset(&mut self) {
		self.points.clear();
		self.value.clear();
	}
}
