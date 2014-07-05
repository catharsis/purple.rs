#![crate_id = "purple#0.0.1"]
#![crate_type = "lib"]
#![feature(globs)]
use self::ll::*;
pub mod ll;
pub fn debug_set_enabled(debug: bool) -> () {
	unsafe {
		purple_debug_set_enabled(debug);
	}
}

pub fn debug_is_enabled() -> bool {
	unsafe {
		purple_debug_is_enabled()
	}
}

#[test]
fn test_debug() {
	let d = debug_is_enabled();
	assert!(d == false);
	debug_set_enabled(true);
	let d = debug_is_enabled();
	assert!(d == true);
}
