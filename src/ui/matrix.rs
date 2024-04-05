use ratatui::{
	layout::{Constraint, Layout, Rect},
	style::{Color, Style},
	symbols,
	text::Text,
	widgets::{Borders, Paragraph},
	Frame,
};

use super::utils::rounded_block;
use crate::state::TetrominoKind;

pub fn matrix<'a, F, const Y: usize, const X: usize>(
	f: &mut Frame,
	rect: Rect,
	v_constraints: [Constraint; Y],
	h_constraints: [Constraint; X],
	hidden_none: bool,
	tm_kind_getter: F,
) where
	F: Fn(usize, usize) -> &'a TetrominoKind,
{
	let vertical_chunks = Layout::vertical(v_constraints).areas::<Y>(rect);

	for (y, vertical_area) in vertical_chunks.into_iter().enumerate() {
		let horinzontal_chunks =
			Layout::horizontal(h_constraints).areas::<X>(vertical_area);

		for (x, horizontal_area) in horinzontal_chunks.into_iter().enumerate() {
			let tetromino_kind = tm_kind_getter(x, y);

			let style = create_style(tetromino_kind);

			let mut b = rounded_block(None).border_style(style);

			if hidden_none && matches!(tetromino_kind, TetrominoKind::None) {
				b = b.borders(Borders::NONE);
			}

			#[cfg(feature = "_dev")]
			{
				b = b
					.title_top(x.to_string())
					.title_bottom((y + vertical_chunks.len()).to_string());
			}

			let p = Paragraph::new(create_text(tetromino_kind))
				.block(b)
				.style(style);

			f.render_widget(p, horizontal_area);
		}
	}
}

fn create_style(tm_kind: &TetrominoKind) -> Style {
	match tm_kind {
		TetrominoKind::I => Style::new().fg(Color::Indexed(14)),
		TetrominoKind::O => Style::new().fg(Color::Indexed(208)),
		TetrominoKind::T => Style::new().fg(Color::Indexed(13)),
		TetrominoKind::L => Style::new().fg(Color::Indexed(202)),
		TetrominoKind::J => Style::new().fg(Color::Indexed(12)),
		TetrominoKind::S => Style::new().fg(Color::Indexed(10)),
		TetrominoKind::Z => Style::new().fg(Color::Indexed(9)),
		TetrominoKind::None => Style::new().fg(Color::Black),
		TetrominoKind::Ghost => Style::new().fg(Color::White),
	}
}

fn create_text(tm_kind: &TetrominoKind) -> Text<'static> {
	let content =
		if matches!(tm_kind, TetrominoKind::None | TetrominoKind::Ghost) {
			' '
		} else {
			symbols::half_block::UPPER
		};

	#[cfg(feature = "_dev")]
	let content = char::from(*tm_kind);

	Text::raw(content.to_string())
}
