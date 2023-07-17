use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
	pub general: General,
	pub keybinds: KeyBinds,
}

#[derive(Debug, Deserialize)]
pub struct General {
	pub mod_key: String,
	pub log_location: String,
}

#[derive(Debug, Deserialize)]
pub struct KeyBinds {
	pub bind: Vec<Bind>,
}

#[derive(Debug, Deserialize)]
pub struct Bind {
	pub key: String,
	pub command: String,
}
