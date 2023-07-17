mod libs;
use input::{
	event::{
		keyboard::{
			KeyState,
			KeyboardEventTrait,
		},
		KeyboardEvent::Key,
	},
	Event,
	Libinput,
};
use libs::{
	open_device::Interface,
	parse_config::parse_config,
};

fn main() {
	let mut input = Libinput::new_with_udev(Interface);
	input.udev_assign_seat("seat0").unwrap();
	let config = parse_config();

	let config_keys: Vec<Vec<u32>> = config
		.keybinds
		.bind
		.iter()
		.map(|keybind| {
			let replaced_keys =
				keybind.key.replace("$mod", "125").replace("return", "37").replace("space", "26");
			let keys =
				replaced_keys.split("+").map(|part| part.parse::<u32>().unwrap_or(0)).collect();
			keys
		})
		.collect();
	println!("{:?}", config_keys);
	let mut key = Vec::new();

	loop {
		input.dispatch().unwrap();
		for event in input.clone().into_iter() {
			if let Event::Keyboard(Key(event)) = event {
				if event.key_state() == KeyState::Pressed {
					key.push(event.key());
				}
				if event.key_state() == KeyState::Released {
					for keybind in &config_keys {
						let index = config_keys.iter().position(|x| x == keybind).unwrap();
						if &key == keybind {
							println!("match. command: {}", config.keybinds.bind[index].command);
						} else {
							println!("no match")
						}
					}
					if key.len() != 0 {
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
