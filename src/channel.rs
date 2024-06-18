use tokio::sync::mpsc::{
	unbounded_channel, UnboundedReceiver, UnboundedSender,
};

pub type Receiver = UnboundedReceiver<Event>;

pub type Sender = UnboundedSender<Event>;

pub enum Event {
	Key(KeyEvent),
	FocusLost,
	GravityReset,
	Gravity,
	Pause,
	PauseCancel,
	LockRefresh,
	LockEnd,
	LockReset,
	LevelChange(u32),
	CountDownStart(u8),
	CountDown(u8),
	Blink,
}

pub enum KeyEvent {
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
}

pub fn channel() -> (Sender, Receiver) {
	unbounded_channel()
}
