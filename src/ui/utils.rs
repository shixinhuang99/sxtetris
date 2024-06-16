use ratatui::{
	layout::{Alignment, Constraint, Flex, Layout, Rect},
	widgets::{block::Title, Block, BorderType, Borders},
};

pub fn centered_rect(
	rect: Rect,
	width: Constraint,
	height: Constraint,
) -> Rect {
	let horizontal_layout = Layout::horizontal([width]).flex(Flex::Center);
	let vertical_layout = Layout::vertical([height]).flex(Flex::Center);

	vertical_layout.areas::<1>(horizontal_layout.areas::<1>(rect)[0])[0]
}

pub fn rounded_block(title: Option<&str>) -> Block {
	let mut b = Block::new()
		.borders(Borders::ALL)
		.border_type(BorderType::Rounded);

	if let Some(title) = title {
		b = b.title(Title::from(title).alignment(Alignment::Center));
	}

	b
}
