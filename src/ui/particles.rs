use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget, Frame};

use crate::state::particles::Particles;

struct ParticlesWidget;

impl StatefulWidget for ParticlesWidget {
	type State = Particles;

	fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
		let mut idxs: Vec<usize> = vec![];

		for (i, particle) in state.iter().enumerate() {
			let (x, y) = particle.pos();

			if x <= area.x
				|| x >= area.x + area.width
				|| y <= area.y
				|| y >= area.y + area.height
			{
				idxs.push(i);
				continue;
			}

			let cell = buf.get_mut(x, y);

			cell.set_char(particle.char);
			cell.set_fg(particle.color);
		}

		state.remove(idxs);
	}
}

pub fn particles(f: &mut Frame, rect: Rect, particles: &mut Particles) {
	let particles_widget = ParticlesWidget;
	f.render_stateful_widget(particles_widget, rect, particles);
}
