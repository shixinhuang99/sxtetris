use crossterm::event::{
	Event, EventStream, KeyCode, KeyEventKind, KeyModifiers,
};
use futures::StreamExt;
use tokio::{
	sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
	time::{interval, sleep, Duration, Instant, Interval},
};

type Sender = UnboundedSender<GameEvent>;
type Receiver = UnboundedReceiver<GameEvent>;
type SubSender = UnboundedSender<SubEvent>;
type SubReceiver = UnboundedReceiver<SubEvent>;

const GRAVITY_LEVEL_LIMIT: u32 = 15;

#[derive(PartialEq)]
pub enum GameEvent {
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
	CountDown(u8),
	Blink,
}

enum SubEvent {
	Pause,
	PauseCancel,
	GravityReset,
	Level(u32),
	LockReset,
	LockRefresh,
}

pub struct MainHandler {
	tx: Sender,
	rx: Receiver,
}

impl MainHandler {
	pub fn new() -> Self {
		let (tx, rx) = unbounded_channel();

		tokio::spawn(term_task(tx.clone()));

		Self {
			tx,
			rx,
		}
	}

	pub async fn recv(&mut self) -> Option<GameEvent> {
		self.rx.recv().await
	}

	pub fn create_sub_handler(&self) -> SubHandler {
		SubHandler::new(self.tx.clone())
	}
}

#[derive(Clone)]
pub struct SubHandler {
	tx: Sender,
	sub_tx: SubSender,
}

impl SubHandler {
	fn new(tx: Sender) -> Self {
		let (sub_tx, sub_rx) = unbounded_channel();

		tokio::spawn(gravity_and_lock_task(tx.clone(), sub_rx));

		Self {
			tx,
			sub_tx,
		}
	}

	pub fn count_down_task(&self, cnt: u8) {
		tokio::spawn(count_down_task(self.tx.clone(), cnt));
	}

	pub fn reset_gravity(&self) {
		self.sub_tx.send(SubEvent::GravityReset).unwrap();
	}

	pub fn change_level(&self, level: u32) {
		self.sub_tx.send(SubEvent::Level(level)).unwrap();
	}

	pub fn reset_lock(&self) {
		self.sub_tx.send(SubEvent::LockReset).unwrap();
	}

	pub fn refresh_lock(&self) {
		self.sub_tx.send(SubEvent::LockRefresh).unwrap();
	}

	pub fn pause(&self) {
		self.sub_tx.send(SubEvent::Pause).unwrap();
	}

	pub fn cancel_pause(&self) {
		self.sub_tx.send(SubEvent::PauseCancel).unwrap();
	}
}

async fn term_task(tx: Sender) {
	let mut event_stream = EventStream::new();

	while let Some(Ok(event)) = event_stream.next().await {
		let game_event = match event {
			Event::Key(key) if key.kind == KeyEventKind::Press => {
				match key.code {
					KeyCode::Char('c')
						if key.modifiers == KeyModifiers::CONTROL =>
					{
						GameEvent::CtrlC
					}
					KeyCode::Up | KeyCode::Char('i') => GameEvent::Up,
					KeyCode::Down | KeyCode::Char('k') => GameEvent::Down,
					KeyCode::Left | KeyCode::Char('j') => GameEvent::Left,
					KeyCode::Right | KeyCode::Char('l') => GameEvent::Right,
					KeyCode::Enter => GameEvent::Enter,
					KeyCode::Char(' ') => GameEvent::Space,
					KeyCode::Esc => GameEvent::Esc,
					KeyCode::Char('p') => GameEvent::P,
					KeyCode::Char('z') => GameEvent::Z,
					_ => continue,
				}
			}
			Event::FocusLost => GameEvent::FocusLost,
			_ => continue,
		};
		tx.send(game_event).unwrap();
	}
}

async fn count_down_task(tx: Sender, cnt: u8) {
	for n in (0..cnt).rev() {
		sleep(Duration::from_secs(1)).await;
		tx.send(GameEvent::CountDown(n)).unwrap();
	}
}

async fn gravity_and_lock_task(tx: Sender, mut sub_rx: SubReceiver) {
	let mut paused = true;
	let mut paused_instant = Instant::now();

	let mut gravity_interval = interval(gravity_duration(1));
	let mut gravity_instant = Instant::now();

	let mut lock = false;
	let mut lock_limit = 15;
	let mut lock_interval = interval(Duration::from_millis(500));
	let mut lock_instant = Instant::now();

	let mut blink_interval = interval(Duration::from_millis(150));
	let mut blink_instant = Instant::now();

	let mut level = 1;

	loop {
		tokio::select! {
			Some(event) = sub_rx.recv() => {
				match event {
					SubEvent::Pause => {
						paused_instant = Instant::now();
						paused = true;
					}
					SubEvent::PauseCancel => {
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
					SubEvent::GravityReset => {
						gravity_interval.reset();
						gravity_instant = Instant::now();
					}
					SubEvent::Level(lv) => {
						if level == lv {
							continue;
						}
						level = lv;
						if level > GRAVITY_LEVEL_LIMIT {
							continue;
						}
						gravity_interval = interval(gravity_duration(level));
					}
					SubEvent::LockReset => {
						lock = false;
						lock_limit = 15;
						lock_interval.reset();
						lock_instant = Instant::now();
					}
					SubEvent::LockRefresh => {
						if lock_limit > 0 {
							lock = true;
							lock_limit -= 1;
							lock_interval.reset();
							lock_instant = Instant::now();
						}
					}
				}
			}
			_ = gravity_interval.tick() => {
				if paused || lock || level >= GRAVITY_LEVEL_LIMIT {
					continue;
				}
				gravity_instant = Instant::now();
				tx.send(GameEvent::Gravity).unwrap();
			}
			_ = lock_interval.tick() => {
				if !lock || paused {
					continue;
				}
				lock = false;
				lock_limit = 15;
				lock_interval.reset();
				lock_instant = Instant::now();
				tx.send(GameEvent::LockEnd).unwrap();
			}
			_ = blink_interval.tick() => {
				if !lock || paused {
					continue;
				}
				blink_instant = Instant::now();
				tx.send(GameEvent::Blink).unwrap();
			}
		}
	}
}

fn gravity_duration(level: u32) -> Duration {
	if level >= GRAVITY_LEVEL_LIMIT {
		return Duration::from_secs(3600);
	}

	let base = (level - 1) as f32;
	let duration_secs = (0.8 - base * 0.007).powf(base);

	#[cfg(feature = "_dev")]
	log::trace!("gravity duration secs: {}", duration_secs);

	Duration::from_secs_f32(duration_secs)
}

fn make_time_continue(
	paused_instant: &Instant,
	target_instant: &Instant,
	interval: &mut Interval,
) {
	let past_time = *paused_instant - *target_instant;
	let period = interval.period();
	if past_time < period {
		interval.reset_at(Instant::now() + (period - past_time));
	}
}
