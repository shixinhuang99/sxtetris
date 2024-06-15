use tokio::sync::mpsc::{
	unbounded_channel, UnboundedReceiver, UnboundedSender,
};

pub type Receiver = UnboundedReceiver<Event>;

pub type Sender = UnboundedSender<Event>;

pub enum Event {
	Key(KeyEvent),
	FocusLost,
	AutoDropStart,
	Pause,
	CancelPause,
	AutoDrop,
	LevelUp(u8),
	LockReset,
	LockEnd,
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
