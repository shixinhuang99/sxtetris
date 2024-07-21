use std::slice::Iter;

use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(transparent)]
pub struct Scores(Vec<u32>);

impl Scores {
	pub fn new() -> Self {
		Self(vec![0; 10])
	}

	pub fn push_new_score(&mut self, new_score: u32) -> Option<usize> {
		self.0.push(new_score);
		self.0.sort_unstable_by(|a, b| b.cmp(a));
		self.0.truncate(10);
		self.0.iter().position(|v| new_score == *v)
	}

	pub fn iter(&self) -> Iter<u32> {
		self.0.iter()
	}
}
