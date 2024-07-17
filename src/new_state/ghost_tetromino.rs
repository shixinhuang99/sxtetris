use crate::common::{Position, TetrominoKind};

#[derive(Default)]
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
