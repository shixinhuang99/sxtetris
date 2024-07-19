use crate::common::{Position, TetrominoKind};

#[derive(Default)]
pub struct GhostTetromino {
	pub kind: TetrominoKind,
	pub position: Position,
}

impl GhostTetromino {
	pub fn hidden(&mut self) {
		self.position = Position::default();
	}
}
