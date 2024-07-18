use crossterm::event::{
	Event as TermEvent, EventStream, KeyCode, KeyEventKind, KeyModifiers,
};
use tokio::{
	sync::{
		broadcast,
		mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
	},
	time::{interval, sleep, Duration, Instant, Interval},
};

use crate::{
	consts::FRAME_RATE_SECS,
	global::{is_locked, is_paused, set_locked, set_paused},
};

type Sender = UnboundedSender<Event>;
type Receiver = UnboundedReceiver<Event>;
type SubSender = broadcast::Sender<SubEvent>;
type SubReceiver = broadcast::Receiver<SubEvent>;

#[derive(PartialEq, Eq)]
pub enum Event {
	Tick,
	FocusLost,
	CtrlC,
	Up,
	Down,
	Left,
	Right,
	Space,
	Enter,
	Esc,
	P,
	Z,
	Gravity,
	LockEnd,
	CountDown,
	Blink,
}

#[derive(Clone, Debug)]
enum SubEvent {
	Pause,
	PauseCancel,
	GravityReset,
	GravityCancel,
	Level(u32),
	LockCancel,
	LockRefresh,
}

pub struct MainHandler {
	tx: Sender,
	rx: Receiver,
}

impl MainHandler {
	pub fn new() -> Self {
		let (tx, rx) = unbounded_channel();

		tokio::spawn(tick_task(tx.clone()));
		tokio::spawn(term_task(tx.clone()));

		Self {
			tx,
			rx,
		}
	}

	pub async fn recv(&mut self) -> Option<Event> {
		self.rx.recv().await
	}

	pub fn create_sub_handler(&self) -> SubHandler {
		SubHandler::new(self.tx.clone())
	}
}

pub struct SubHandler {
	tx: Sender,
	sub_tx: SubSender,
	_sub_rx: SubReceiver,
}

impl SubHandler {
	fn new(tx: Sender) -> Self {
		let (sub_tx, _sub_rx) = broadcast::channel(100);

		Self {
			tx,
			sub_tx,
			_sub_rx,
		}
	}

	pub fn spawn_count_down_task(&self, cnt: u8) {
		tokio::spawn(count_down_task(self.tx.clone(), cnt));
	}

	pub fn spawn_gravity_task(&self) {
		tokio::spawn(gravity_task(self.tx.clone(), self.sub_tx.subscribe()));
	}

	pub fn spawn_lock_task(&self) {
		set_locked(true);
		tokio::spawn(lock_task(self.tx.clone(), self.sub_tx.subscribe()));
	}

	fn send(&self, event: SubEvent) {
		self.sub_tx.send(event).unwrap();
	}

	pub fn reset_gravity(&self) {
		self.send(SubEvent::GravityReset);
	}

	pub fn cancel_grvity(&self) {
		self.send(SubEvent::GravityCancel);
	}

	pub fn change_level(&self, level: u32) {
		self.send(SubEvent::Level(level));
	}

	pub fn cancel_lock(&self) {
		set_locked(false);
		self.send(SubEvent::LockCancel);
	}

	pub fn refresh_lock(&self) {
		self.send(SubEvent::LockRefresh);
	}

	pub fn pause(&mut self) {
		set_paused(true);
		self.send(SubEvent::Pause);
	}

	pub fn cancel_pause(&self) {
		set_paused(false);
		self.send(SubEvent::PauseCancel);
	}
}

async fn tick_task(tx: Sender) {
	let mut tick_interval = interval(Duration::from_secs_f32(FRAME_RATE_SECS));

	loop {
		tick_interval.tick().await;
		tx.send(Event::Tick).unwrap();
	}
}

async fn term_task(tx: Sender) {
	use futures_util::StreamExt;

	let mut event_stream = EventStream::new();

	let space_throttle_ms = Duration::from_millis(200);
	let mut space_instant = Instant::now();
	let mut is_last_key_space = false;

	while let Some(Ok(event)) = event_stream.next().await {
		let game_event = match event {
			TermEvent::Key(key) if key.kind == KeyEventKind::Press => {
				let e = match key.code {
					KeyCode::Char('c')
						if key.modifiers == KeyModifiers::CONTROL =>
					{
						Event::CtrlC
					}
					KeyCode::Up | KeyCode::Char('i') => Event::Up,
					KeyCode::Down | KeyCode::Char('k') => Event::Down,
					KeyCode::Left | KeyCode::Char('j') => Event::Left,
					KeyCode::Right | KeyCode::Char('l') => Event::Right,
					KeyCode::Enter => Event::Enter,
					KeyCode::Char(' ') => {
						if is_last_key_space
							&& Instant::now() - space_instant
								< space_throttle_ms
						{
							space_instant = Instant::now();
							continue;
						}

						Event::Space
					}
					KeyCode::Esc => Event::Esc,
					KeyCode::Char('p') => Event::P,
					KeyCode::Char('z') => Event::Z,
					_ => continue,
				};

				if e == Event::Space {
					is_last_key_space = true;
					space_instant = Instant::now();
				} else {
					is_last_key_space = false;
				}

				e
			}
			TermEvent::FocusLost => Event::FocusLost,
			_ => continue,
		};
		tx.send(game_event).unwrap();
	}
}

async fn count_down_task(tx: Sender, cnt: u8) {
	for _ in 0..cnt {
		sleep(Duration::from_secs(1)).await;
		tx.send(Event::CountDown).unwrap();
	}
}

async fn gravity_task(tx: Sender, mut sub_rx: SubReceiver) {
	const MAX_GRAVITY_LEVEL: u32 = 15;

	let mut paused_instant = Instant::now();

	let mut level = 1;

	let mut gravity_interval = interval(gravity_duration(level));
	let mut gravity_instant = Instant::now();

	gravity_interval.reset();

	loop {
		tokio::select! {
			Ok(event) = sub_rx.recv() => {
				match event {
					SubEvent::Pause => {
						paused_instant = Instant::now();
					}
					SubEvent::PauseCancel => {
						make_time_continue(
							&mut gravity_interval,
							&paused_instant,
							&gravity_instant,
						);
					}
					SubEvent::GravityCancel => {
						break;
					}
					SubEvent::GravityReset => {
						gravity_interval.reset();
						gravity_instant = Instant::now();
					}
					SubEvent::Level(lv) => {
						level = lv;
						if level <= MAX_GRAVITY_LEVEL {
							gravity_interval = interval(gravity_duration(level));
							gravity_interval.reset();
						}
					}
					_ => (),
				}
			}
			_ = gravity_interval.tick() => {
				if is_paused() || is_locked() {
					continue;
				}
				gravity_instant = Instant::now();
				tx.send(Event::Gravity).unwrap();
			}
		}
	}
}

async fn lock_task(tx: Sender, mut sub_rx: SubReceiver) {
	let mut paused_instant = Instant::now();

	let mut lock_limit = 15;
	let mut lock_interval = interval(Duration::from_millis(500));
	let mut lock_instant = Instant::now();

	let mut blink_interval = interval(Duration::from_millis(150));
	let mut blink_instant = Instant::now();

	lock_interval.reset();

	loop {
		tokio::select! {
			Ok(event) = sub_rx.recv() => {
				match event {
					SubEvent::Pause => {
						paused_instant = Instant::now();
					}
					SubEvent::PauseCancel => {
						make_time_continue(
							&mut lock_interval,
							&paused_instant,
							&lock_instant,
						);
						make_time_continue(
							&mut blink_interval,
							&paused_instant,
							&blink_instant,
						);
					}
					SubEvent::LockCancel => {
						break;
					}
					SubEvent::LockRefresh => {
						if lock_limit > 0 {
							lock_limit -= 1;
							lock_interval.reset();
							lock_instant = Instant::now();
						}
					}
					_ => (),
				}
			}
			_ = lock_interval.tick() => {
				if is_paused() {
					continue;
				}
				set_locked(false);
				tx.send(Event::LockEnd).unwrap();
				break;
			}
			_ = blink_interval.tick() => {
				if is_paused() {
					continue;
				}
				blink_instant = Instant::now();
				tx.send(Event::Blink).unwrap();
			}
		}
	}
}

fn gravity_duration(level: u32) -> Duration {
	let base = (level - 1) as f32;
	let duration_secs = (0.8 - base * 0.007).powf(base);

	Duration::from_secs_f32(duration_secs)
}

fn make_time_continue(
	interval: &mut Interval,
	paused_instant: &Instant,
	instant: &Instant,
) {
	let past_time = *paused_instant - *instant;
	let period = interval.period();
	if past_time < period {
		interval.reset_at(Instant::now() + (period - past_time));
	}
}
