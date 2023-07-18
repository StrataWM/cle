use crate::libs::{
	get_key_code::get_key_code,
	structs::Config,
};

pub fn parse_keycodes(config: &Config) -> Vec<Vec<u32>> {
	let mod_key_code = get_key_code(&config.general.mod_key);
	let mut key_codes = Vec::new();

	for bind in &config.keybinds.bind {
		let mut keys: Vec<&str> = bind.keys.split('+').collect();
		let mut key_codes_bind = Vec::new();

		for key in &mut keys {
			if *key == "$mod" {
				*key = &config.general.mod_key;
			}

			let key_code_value = get_key_code(key);
			if key_code_value != 0 {
				key_codes_bind.push(key_code_value);
			} else {
				continue;
			}
		}

		key_codes.push(key_codes_bind);
	}

	key_codes
}
