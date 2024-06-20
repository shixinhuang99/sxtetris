use std::sync::atomic::{AtomicBool, Ordering::Relaxed};

use crossterm::event::{
	Event, EventStream, KeyCode, KeyEventKind, KeyModifiers,
};
use futures::StreamExt;
use tokio::{
	sync::{
		broadcast,
		mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
	},
	time::{interval, sleep, Duration, Instant, Interval},
};

type Sender = UnboundedSender<GameEvent>;
type Receiver = UnboundedReceiver<GameEvent>;
type SubSender = broadcast::Sender<SubEvent>;
type SubReceiver = broadcast::Receiver<SubEvent>;

const MAX_GRAVITY_LEVEL: u32 = 15;

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

static PAUSED: AtomicBool = AtomicBool::new(false);

static LOCKED: AtomicBool = AtomicBool::new(false);

pub fn is_paused() -> bool {
	PAUSED.load(Relaxed)
}

pub fn is_locked() -> bool {
	LOCKED.load(Relaxed)
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
		LOCKED.store(true, Relaxed);
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
		LOCKED.store(false, Relaxed);
		self.send(SubEvent::LockCancel);
	}

	pub fn refresh_lock(&self) {
		self.send(SubEvent::LockRefresh);
	}

	pub fn pause(&mut self) {
		PAUSED.store(true, Relaxed);
		self.send(SubEvent::Pause);
	}

	pub fn cancel_pause(&mut self) {
		PAUSED.store(false, Relaxed);
		self.send(SubEvent::PauseCancel);
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

async fn gravity_task(tx: Sender, mut sub_rx: SubReceiver) {
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
				tx.send(GameEvent::Gravity).unwrap();
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
				LOCKED.store(false, Relaxed);
				tx.send(GameEvent::LockEnd).unwrap();
				break;
			}
			_ = blink_interval.tick() => {
				if is_paused() {
					continue;
				}
				blink_instant = Instant::now();
				tx.send(GameEvent::Blink).unwrap();
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
