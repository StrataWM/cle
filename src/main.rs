use input::{
	event::{
		keyboard::{
			KeyState,
			KeyboardEventTrait,
		},
		KeyboardEvent,
		KeyboardEvent::Key,
	},
	Event,
	Libinput,
	LibinputInterface,
};
use libc::{
	O_RDONLY,
	O_RDWR,
	O_WRONLY,
};
use std::{
	fs::{
		File,
		OpenOptions,
	},
	os::unix::{
		fs::OpenOptionsExt,
		io::OwnedFd,
	},
	path::Path,
};
struct Interface;

impl LibinputInterface for Interface {
	fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
		OpenOptions::new()
			.custom_flags(flags)
			.read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
			.write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
			.open(path)
			.map(|file| file.into())
			.map_err(|err| err.raw_os_error().unwrap())
	}
	fn close_restricted(&mut self, fd: OwnedFd) {
		drop(File::from(fd));
	}
}

fn main() {
	let mut input = Libinput::new_with_udev(Interface);
	input.udev_assign_seat("seat0").unwrap();
	let mut key = Vec::new();

	loop {
		input.dispatch().unwrap();
		for event in input.clone().into_iter() {
			if let Event::Keyboard(Key(event)) = event {
				if event.key_state() == KeyState::Pressed {
					// println!("{:?}", event.key());
					key.push(event.key());
					// continue;
				}
				if event.key_state() == KeyState::Released {
					if (key == [125, 28]) {
						println!("Launcing terminal");
					} else {
						println!("Unknown binding");
					}
					if (key.len() != 0) {
						key.clear();
					} else {
						continue;
					}
				}
			} else {
				continue;
			}
			println!("{:?}", key);
		}
	}
}
