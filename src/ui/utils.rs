use ratatui::{
	layout::{Alignment, Constraint, Flex, Layout, Rect},
	style::{Style, Stylize},
	widgets::{Block, BorderType, Clear, Padding},
	Frame,
};

pub fn centered_rect(
	rect: Rect,
	width: Constraint,
	height: Constraint,
) -> Rect {
	let h_layout = Layout::horizontal([width]).flex(Flex::Center);
	let v_layout = Layout::vertical([height]).flex(Flex::Center);

	v_layout.areas::<1>(h_layout.areas::<1>(rect)[0])[0]
}

pub fn rounded_block() -> Block<'static> {
	Block::bordered()
		.border_type(BorderType::Rounded)
		.style(Style::new().on_black())
		.border_style(Style::new().white())
		.title_alignment(Alignment::Center)
		.title_style(Style::new().bold())
}

pub struct Popup {
	width: u16,
	height: u16,
	title: Option<&'static str>,
	padding: Option<Padding>,
}

impl Popup {
	pub fn new(width: u16, height: u16) -> Self {
		Self {
			width,
			height,
			title: None,
			padding: None,
		}
	}

	pub fn title(mut self, title: &'static str) -> Self {
		self.title = Some(title);
		self
	}

	pub fn padding(mut self, padding: Padding) -> Self {
		self.padding = Some(padding);
		self
	}

	pub fn render(self, f: &mut Frame) -> Rect {
		let area = centered_rect(
			f.size(),
			Constraint::Length(self.width),
			Constraint::Length(self.height),
		);

		f.render_widget(Clear, area);

		let mut block = rounded_block();

		if let Some(title) = self.title {
			block = block.title(title);
		}

		if let Some(padding) = self.padding {
			block = block.padding(padding);
		}

		let block_inner = block.inner(area);

		f.render_widget(block, area);

		block_inner
	}
}
