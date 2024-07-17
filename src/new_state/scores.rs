use std::slice::Iter;

pub struct Scores(Vec<u32>);

impl Scores {
	pub fn new() -> Self {
		Self(vec![0; 10])
	}

	pub fn push_new_score(&mut self, new_score: u32) -> Option<usize> {
		if let Some(i) = self.0.iter().position(|v| new_score >= *v) {
			self.0[i] = new_score;
			Some(i)
		} else {
			None
		}
	}

	pub fn iter(&self) -> Iter<u32> {
		self.0.iter()
	}
}
