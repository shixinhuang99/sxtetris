use super::tetromino_kind::TetrominoKind;

pub trait Board {
	fn get_cell(&self, x: usize, y: usize) -> Option<&TetrominoKind>;
}
