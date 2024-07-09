use super::{point::Point, tetromino_kind::TetrominoKind};

pub trait Board {
	fn get_cell_kind(&self, x: usize, y: usize) -> Option<&TetrominoKind>;
}

pub struct Cell {
	pub kind: TetrominoKind,
	pub point: Point,
}
