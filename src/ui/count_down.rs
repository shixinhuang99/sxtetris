use ratatui::{
	style::{Color, Style},
	text::Line,
	widgets::block::Padding,
	Frame,
};
use tui_big_text::{BigText, PixelSize};

use super::utils::Popup;

pub fn count_down(f: &mut Frame, count_down: u8) {
	let popup = Popup::new(16, 14)
		.title("PAUSED")
		.padding(Padding::new(4, 4, 2, 3))
		.render(f);

	let text = BigText::builder()
		.pixel_size(PixelSize::Full)
		.lines([Line::raw(count_down.to_string())])
		.style(Style::new().fg(Color::White))
		.build()
		.unwrap();

	f.render_widget(text, popup);
}
