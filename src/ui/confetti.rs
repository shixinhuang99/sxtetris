use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};

use crate::state::ConfettiState;

pub struct Confetti;

impl StatefulWidget for Confetti {
	type State = ConfettiState;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
		let mut indices_to_remove = Vec::new();

		for (i, particle) in state.particles.iter().enumerate() {
			let (x, y) = particle.pos();

			if x == 0 || x >= area.width || y == 0 || y >= area.height {
				indices_to_remove.push(i);
				continue;
			}

			let cell = buf.get_mut(x, y);
			cell.set_char(particle.char);
			cell.set_fg(particle.color);
		}

		state.remove_particles(indices_to_remove);
	}
}
