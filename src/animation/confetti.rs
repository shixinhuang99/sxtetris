// source: https://github.com/Handfish/confetty_rs

use fastrand::f32 as rf32;
use ratatui::{
	buffer::Buffer, layout::Rect, style::Color, widgets::StatefulWidget,
};

use crate::{color, consts::FRAME_RATE_SECS};

struct Particle {
	char: char,
	color: Color,
	x: f32,
	y: f32,
	x_velocity: f32,
	y_velocity: f32,
}

impl Particle {
	fn new(x: u16, y: u16, dir: ConfettiDirection) -> Self {
		use ConfettiDirection::*;

		let x_velocity = match dir {
			Top | Bottom => rf32() * 50.0,
			Right => 60.0 + rf32() * 50.0,
			Left => -60.0 + rf32() * 50.0,
		};

		Particle {
			char: random_character(),
			color: random_color(),
			x: x as f32,
			y: y as f32,
			x_velocity,
			y_velocity: -5.0 + rf32() * 10.0,
		}
	}

	fn update(&mut self) {
		self.x += self.x_velocity * FRAME_RATE_SECS;
		self.y += self.y_velocity * FRAME_RATE_SECS;
		self.y_velocity += 9.8 * FRAME_RATE_SECS;
	}

	fn pos(&self) -> (u16, u16) {
		(self.x.floor() as u16, self.y.floor() as u16)
	}
}

fn random_character() -> char {
	const CHARACTERS: [char; 6] = ['█', '▓', '▒', '░', '▄', '▀'];
	fastrand::choice(CHARACTERS).unwrap_or(CHARACTERS[0])
}

fn random_color() -> Color {
	const COLORS: [Color; 7] = [
		color::red(),
		color::orange(),
		color::yellow(),
		color::green(),
		color::cyan(),
		color::blue(),
		color::purple(),
	];
	fastrand::choice(COLORS).unwrap_or(COLORS[0])
}

pub struct ConfettiState {
	points: Vec<ConfettiPoint>,
	particles: Vec<Particle>,
}

struct ConfettiPoint {
	x: usize,
	y: usize,
	dir: ConfettiDirection,
}

#[derive(Clone, Copy)]
pub enum ConfettiDirection {
	Top,
	Right,
	Bottom,
	Left,
}

impl ConfettiState {
	pub fn new() -> Self {
		Self {
			points: Vec::new(),
			particles: Vec::new(),
		}
	}

	pub fn push_tm_point(
		&mut self,
		x: usize,
		y: usize,
		dir: ConfettiDirection,
	) {
		self.points.push(ConfettiPoint {
			x,
			y,
			dir,
		});
	}

	pub fn get_point_dirs(
		&mut self,
		x: usize,
		y: usize,
	) -> Vec<ConfettiDirection> {
		let mut dirs = Vec::new();
		let mut idxs = Vec::new();

		for (i, p) in self.points.iter().enumerate() {
			if p.x == x && p.y == y {
				dirs.push(p.dir);
				idxs.push(i);
			}
		}

		for i in idxs.into_iter().rev() {
			self.points.swap_remove(i);
		}

		dirs
	}

	pub fn move_particles(&mut self) {
		for p in self.particles.iter_mut() {
			p.update();
		}
	}

	pub fn spawn_particles(
		&mut self,
		x: u16,
		y: u16,
		width: u16,
		height: u16,
		dirs: Vec<ConfettiDirection>,
	) {
		use ConfettiDirection::*;

		for dir in dirs {
			match dir {
				Top => {
					for cx in x..(x + width) {
						self.particles.push(Particle::new(cx, y, dir));
					}
				}
				Right => {
					for cy in y..(y + height) {
						self.particles.push(Particle::new(x + width, cy, dir));
					}
				}
				Bottom => {
					for cx in x..(x + width) {
						self.particles.push(Particle::new(cx, y + height, dir));
					}
				}
				Left => {
					for cy in y..(y + height) {
						self.particles.push(Particle::new(x, cy, dir));
					}
				}
			}
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

pub struct Confetti;

impl StatefulWidget for Confetti {
	type State = ConfettiState;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
		let mut indices_to_remove = Vec::new();

		for (i, particle) in state.particles.iter().enumerate() {
			let (x, y) = particle.pos();

			if x == 0 || x >= area.width || y == 0 || y >= area.height {
				indices_to_remove.push(i);
				continue;
			}

			let cell = buf.get_mut(x, y);
			cell.set_char(particle.char);
			cell.set_fg(particle.color);
		}

		state.remove_particles(indices_to_remove);
	}
}
