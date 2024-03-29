use ratatui::{
	layout::{Constraint, Flex, Layout, Rect},
	style::{Color, Style},
	Frame,
};
use tui_big_text::{BigText, PixelSize};

pub fn list<const N: usize>(
	f: &mut Frame,
	rect: Rect,
	items: [&'static str; N],
	selected: usize,
) {
	let vertical_chunks = Layout::vertical([Constraint::Length(4); N])
		.spacing(2)
		.areas::<N>(rect);

	for (i, item) in items.into_iter().enumerate() {
		let mut builder = BigText::builder();

		builder.pixel_size(PixelSize::Quadrant).lines([item.into()]);

		if i == selected {
			builder.style(Style::new().fg(Color::Yellow));
		}

		let title = builder.build().unwrap();

		let title_area =
			Layout::horizontal([Constraint::Length(item.len() as u16 * 4)])
				.flex(Flex::Center)
				.areas::<1>(vertical_chunks[i])[0];

		f.render_widget(title, title_area);
	}
}
