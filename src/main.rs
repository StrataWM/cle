mod libs;
use crate::libs::parse_keycodes::parse_keycodes;
use chrono::Local;
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
use log::debug;
use std::{
	env::var,
	error::Error,
	io::stdout,
	process::Command,
};
use tracing_subscriber::fmt::writer::MakeWriterExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let mut input = Libinput::new_with_udev(Interface);
	input.udev_assign_seat("seat0").unwrap();
	let config = parse_config();
	let config_keys = parse_keycodes(&config);
	let mut key = Vec::new();

	let log_dir = format!("{}/.strata/kagi/", var("HOME").expect("This variable should be set!!!"));
	let file_appender = tracing_appender::rolling::never(
		&log_dir,
		format!("kagi_{}.log", Local::now().format("%Y-%m-%d_%H:%M:%S")),
	);

	let latest_file_appender = tracing_appender::rolling::never(&log_dir, "latest.log");
	let log_appender = stdout.and(file_appender).and(latest_file_appender);

	if let Ok(env_filter) = tracing_subscriber::EnvFilter::try_from_default_env() {
		tracing_subscriber::fmt().with_writer(log_appender).with_env_filter(env_filter).init();
	} else {
		tracing_subscriber::fmt().with_writer(log_appender).init();
	}

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
							Command::new("/bin/sh")
								.arg("-c")
								.arg(&config.keybinds.bind[index].cmd)
								.spawn()
								.ok();
							debug!(
								"Keybinding matched!!!\n Launching: {}",
								config.keybinds.bind[index].cmd
							);
						} else {
							debug!("Unknown keybinding");
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
		}
	}
}
