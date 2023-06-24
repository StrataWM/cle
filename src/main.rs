mod libs;
use libs::detect_device::detect_device;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let mut d = detect_device();
	println!("{d}");
	println!("Events:");
	loop {
		for ev in d.fetch_events().unwrap() {
			println!("{ev:?}");
		}
	}
}
