use crate::core::{Position, TetrominoKind};

pub struct GhostTetromino {
	pub kind: TetrominoKind,
	pub position: Position,
}

impl GhostTetromino {
	pub fn new(kind: TetrominoKind, position: Position) -> Self {
		Self {
			kind,
			position,
		}
	}
}

impl Default for GhostTetromino {
	fn default() -> Self {
		Self {
			kind: TetrominoKind::I,
			position: Position::default(),
		}
	}
}
