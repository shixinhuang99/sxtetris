use crossterm::event::{
	Event as TermEvent, EventStream, KeyCode, KeyEventKind, KeyModifiers,
};
use futures::StreamExt;
use tokio::time::{interval, sleep, Duration, Instant, Interval};

use crate::channel::{channel, Event, KeyEvent, Receiver, Sender};

pub struct Handler {
	rx: Receiver,
}

impl Handler {
	pub fn new(state_rx: Receiver) -> Self {
		let (tx, rx) = channel();

		tokio::spawn(task(tx.clone(), state_rx));

		Self {
			rx,
		}
	}

	pub async fn next(&mut self) -> Option<Event> {
		self.rx.recv().await
	}
}

async fn task(tx: Sender, mut state_rx: Receiver) {
	let mut event_stream = EventStream::new();

	let mut paused = true;
	let mut paused_instant = Instant::now();

	let mut gravity_interval = interval(gravity_duration(1));
	let mut gravity_instant = Instant::now();

	let mut lock_interval = interval(Duration::from_millis(500));
	let mut lock_limit = 15;
	let mut lock = false;
	let mut lock_instant = Instant::now();

	let mut blink_interval = interval(Duration::from_millis(250));
	let mut blink_instant = Instant::now();

	loop {
		#[cfg(feature = "_dev")]
		log::trace!("tx count: {}", tx.strong_count());

		tokio::select! {
			Some(Ok(term_event)) = event_stream.next() => {
				let event = match term_event {
					TermEvent::Key(key) if key.kind == KeyEventKind::Press => {
						let key_event = match key.code {
							KeyCode::Char('c')
								if key.modifiers == KeyModifiers::CONTROL =>
							{
								KeyEvent::CtrlC
							}
							KeyCode::Up | KeyCode::Char('i') => KeyEvent::Up,
							KeyCode::Down | KeyCode::Char('k') => KeyEvent::Down,
							KeyCode::Left | KeyCode::Char('j') => KeyEvent::Left,
							KeyCode::Right | KeyCode::Char('l') => KeyEvent::Right,
							KeyCode::Enter => KeyEvent::Enter,
							KeyCode::Char(' ') => KeyEvent::Space,
							KeyCode::Esc => KeyEvent::Esc,
							KeyCode::Char('p') => KeyEvent::P,
							KeyCode::Char('z') => KeyEvent::Z,
							_ => continue,
						};
						Event::Key(key_event)
					}
					TermEvent::FocusLost => Event::FocusLost,
					_ => continue,
				};
				if tx.send(event).is_err() {
					break;
				}
			}
			Some(event) = state_rx.recv() => {
				match event {
					Event::Pause => {
						paused_instant = Instant::now();
						paused = true;
					}
					Event::PauseCancel => {
						make_time_continue(
							&paused_instant,
							&gravity_instant,
							&mut gravity_interval,
						);
						make_time_continue(
							&paused_instant,
							&lock_instant,
							&mut lock_interval,
						);
						make_time_continue(
							&paused_instant,
							&blink_instant,
							&mut blink_interval,
						);
						paused = false;
					}
					Event::GravityReset => {
						gravity_interval.reset();
						gravity_instant = Instant::now();
					}
					Event::LevelChange(level) => {
						if level <= 15 {
							gravity_interval = interval(gravity_duration(level));
						}
					}
					Event::LockReset => {
						lock = false;
						lock_limit = 15;
						lock_interval.reset();
						lock_instant = Instant::now();
					}
					Event::LockRefresh => {
						if lock_limit > 0 {
							lock = true;
							lock_limit -= 1;
							lock_interval.reset();
							lock_instant = Instant::now();
						}
					}
					Event::CountDownStart(cnt) => {
						tokio::spawn(count_down_task(tx.clone(), cnt));
					}
					_ => (),
				}
			}
			_ = gravity_interval.tick() => {
				if paused || lock {
					continue;
				}
				gravity_instant = Instant::now();
				if tx.send(Event::Gravity).is_err() {
					break;
				}
			}
			_ = lock_interval.tick() => {
				if !lock || paused {
					continue;
				}
				lock = false;
				lock_limit = 15;
				lock_interval.reset();
				lock_instant = Instant::now();
				if tx.send(Event::LockEnd).is_err() {
					break;
				}
			}
			_ = blink_interval.tick() => {
				if !lock || paused {
					continue;
				}
				blink_instant = Instant::now();
				if tx.send(Event::Blink).is_err() {
					break;
				}
			}
		}
	}
}

async fn count_down_task(tx: Sender, mut cnt: u8) {
	while cnt > 0 {
		sleep(Duration::from_secs(1)).await;
		cnt -= 1;
		tx.send(Event::CountDown(cnt)).unwrap();
	}
}

fn gravity_duration(level: u32) -> Duration {
	let base = (level - 1) as f32;
	let duration_secs = (0.8 - base * 0.007).powf(base);

	#[cfg(feature = "_dev")]
	log::trace!("gravity duration secs: {}", duration_secs);

	Duration::from_secs_f32(duration_secs)
}

fn make_time_continue(
	paused_instant: &Instant,
	instant: &Instant,
	interval: &mut Interval,
) {
	let past_time = *paused_instant - *instant;
	let period = interval.period();
	if past_time < period {
		interval.reset_at(Instant::now() + (period - past_time));
	}
}
