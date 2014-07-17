#![crate_name = "purple"]
#![crate_type = "lib"]
#![feature(globs)]
extern crate libc;
use self::ll::*;
pub mod ll;

mod debug {
	use ll::*;
	pub fn set_enabled(debug: bool) -> () {
		unsafe {
			purple_debug_set_enabled(debug);
		}
	}

	pub fn is_enabled() -> bool {
		unsafe {
			purple_debug_is_enabled()
		}
	}
}
mod core {
	use std::c_str::CString;
	use ll::*;
	pub fn init(ui: &str) -> () {
		unsafe {
			let cstr = ui.to_c_str();
			purple_core_init(cstr.as_ptr());
		}
	}

	pub fn quit() -> () {
		unsafe {
			purple_core_quit();
		}
	}

	pub fn quit_cb(unused:gpointer) -> gboolean {
		unsafe {
			purple_core_quit_cb(unused as gpointer)
		}
	}

	pub fn get_ui() -> String {
		unsafe {
			CString::new(purple_core_get_ui(), false).as_str().unwrap().to_string()
		}
	}

	pub fn get_core() -> *mut PurpleCore {
		unsafe {
			purple_get_core()
		}
	}
	pub fn set_ui_ops(ops: *mut PurpleCoreUiOps) -> () {
		unsafe {
			purple_core_set_ui_ops(ops);
		}
	}

	pub fn get_ui_ops() -> *mut PurpleCoreUiOps {
		unsafe {
			purple_core_get_ui_ops()
		}
	}

	pub fn get_version() -> String {
		unsafe {
			CString::new(purple_core_get_version(), false).as_str().unwrap().to_string()
		}
	}
}

mod eventloop {
	use ll::*;
	pub fn set_ui_ops(ops: *mut PurpleEventLoopUiOps) -> () {
		unsafe {
			purple_eventloop_set_ui_ops(ops);
		}
	}

	pub fn get_ui_ops() -> *mut PurpleEventLoopUiOps {
		unsafe {
			purple_eventloop_get_ui_ops()
		}
	}

	pub fn timeout_add(interval:guint, function:GSourceFunc, data:gpointer) -> guint {
		unsafe {
			purple_timeout_add(interval, function, data)
		}
	}
}
#[test]
fn test_debug() {
	let d = debug::is_enabled();
	assert!(d == false);
	debug::set_enabled(true);
	let d = debug::is_enabled();
	assert!(d == true);
}

#[test]
fn test_core_init() {
	// FIXME: Woo... Bring out your boilerplates
	// TODO: Add assertions that callbacks are being called properly
	use self::libc::{c_int, c_uint};
	let e = "PURPLE_TEST";
	fn timeout_add_fn(_: guint, _: GSourceFunc, _: gpointer) -> c_uint {
		1
	};

	fn timeout_remove_fn(_: guint) -> gboolean {
		1
	};

	fn input_add_fn(_:c_int, _:PurpleInputCondition, _:PurpleInputFunction, _:gpointer) -> guint {
		0
	};

	fn input_remove_fn(_:guint) -> gboolean {
		1
	};

	fn ui_init_fn() -> () {
	};

	let mut core_ops = PurpleCoreUiOps {
		ui_prefs_init: None,
		debug_ui_init: None,
		ui_init: Some(ui_init_fn),
		quit: None,
		get_ui_info: None
	};

	let mut eventloop_ops = PurpleEventLoopUiOps {
		timeout_add: timeout_add_fn,
		timeout_remove: timeout_remove_fn,
		input_add: input_add_fn,
		input_remove: input_remove_fn,
		input_get_error: None,
		timeout_add_seconds: None,
	};

	eventloop::set_ui_ops(&mut eventloop_ops);
	core::set_ui_ops(&mut core_ops);
	core::init(e);
	assert!(e == core::get_ui().as_slice());
}

#[test]
fn test_core_get_version() {
	let version = core::get_version();
	assert!("" != version.as_slice());
}
