use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Scores {
	scores: Vec<u32>,
	#[serde(skip)]
	new_score: Option<String>,
}

impl Scores {
	pub fn new() -> Self {
		Self {
			scores: vec![0; 10],
			new_score: None,
		}
	}

	pub fn push_new_score(&mut self, new_score: u32) {
		if let Some(i) = self.scores.iter().position(|v| new_score >= *v) {
			self.scores[i] = new_score;
			self.new_score = Some(format!("{}.{:>11}", i + 1, new_score));
		} else {
			self.new_score = None;
		}
	}

	pub fn get_new_score(&self) -> Option<&str> {
		self.new_score.as_deref()
	}

	pub fn to_strings(&self) -> Vec<String> {
		self.scores
			.iter()
			.enumerate()
			.map(|(i, score)| {
				if i >= 9 {
					format!("{}.{:>11}", i + 1, score)
				} else {
					format!("{}.{:>12}", i + 1, score)
				}
			})
			.collect()
	}
}
