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
	PauseCancel,
	AutoDrop,
	LevelUp(u8),
	LockRefresh,
	LockEnd,
	LockReset,
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
