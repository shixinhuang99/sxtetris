use crate::save_v2::Saveable;

pub struct Scores {
	scores: Vec<u32>,
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
		if let Some(i) = self.scores.iter().position(|v| new_score <= *v) {
			self.scores[i] = new_score;
			self.new_score = Some(format!("{}.{:>11}", i + 1, new_score));
		}
	}

	pub fn take_new_score(&mut self) -> Option<String> {
		self.new_score.take()
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

	pub fn read_save_v1(&mut self, scores: &[u32]) {
		self.scores = scores.to_vec();
	}
}

impl Saveable for Scores {
	fn get_key(&self) -> &'static str {
		"scores"
	}

	fn get_content(&self) -> String {
		self.scores
			.iter()
			.map(|v| v.to_string())
			.collect::<Vec<String>>()
			.join(" ")
	}

	fn read_content(&mut self, content: &str) {
		for (i, s) in content.split_ascii_whitespace().enumerate() {
			self.scores[i] = s.parse::<u32>().unwrap_or(0);
		}
	}
}
