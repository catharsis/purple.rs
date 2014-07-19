#![crate_name = "purple"]
#![crate_type = "lib"]
#![feature(globs)]
extern crate libc;
use ffi::*;
pub mod ffi;
mod debug {
	use ffi::debug::*;
	use ffi::glibtypes::*;

	pub fn set_enabled(debug: bool) -> () {
		unsafe {
			let d = match debug {
				false => 0,
				true => 1
			};
			purple_debug_set_enabled(d);
		}
	}

	pub fn is_enabled() -> bool {
		unsafe {
			purple_debug_is_enabled() != 0
		}
	}

	pub fn set_verbose(verbose: bool) -> () {
		unsafe {
			let v = match verbose {
				false => 0,
				true => 1
			};
			purple_debug_set_verbose(v)
		}
	}

	pub fn is_verbose() -> bool {
		unsafe {
			purple_debug_is_verbose() != 0
		}
	}

}
mod core {
	use std::c_str::CString;
	use ffi::core::*;
	use ffi::glibtypes::*;
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

	pub fn quit_cb(unused:gpointer) -> bool {
		unsafe {
			0 != purple_core_quit_cb(unused as gpointer)
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

	pub fn migrate() -> bool {
		unsafe {
			0 != purple_core_migrate()
		}
	}

	pub fn ensure_single_instance() -> bool {
		unsafe {
			0 != purple_core_ensure_single_instance()
		}
	}

	pub fn get_ui_info() -> *mut GHashTable {
		unsafe {
			purple_core_get_ui_info()
		}
	}

}

mod eventloop {
	use libc::c_int;
	use ffi::eventloop::*;
	use ffi::glibtypes::*;
	use ffi::types::{PurpleInputCondition, PurpleInputFunction};
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

	pub fn timeout_add_seconds(interval:guint, function:GSourceFunc, data:gpointer) -> guint {
		unsafe {
			purple_timeout_add_seconds(interval, function, data)
		}
	}

	pub fn timeout_remove(handle:guint) -> bool {
		unsafe {
			0 != purple_timeout_remove(handle)
		}
	}
	pub fn input_add(fd:c_int, cond:PurpleInputCondition, function:PurpleInputFunction, user_data:gpointer) -> guint {
		unsafe {
			purple_input_add(fd, cond, function, user_data)
		}
	}

	pub fn input_remove(handle:guint) -> bool {
		unsafe {
			0 != purple_input_remove(handle)
		}
	}

	pub fn input_get_error(fd:c_int, error: *mut c_int) -> c_int {
		unsafe {
			purple_input_get_error(fd, error)
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
fn test_debug_verbose() {
	let d = debug::is_verbose();
	assert!(d == false);
	debug::set_verbose(true);
	let d = debug::is_verbose();
	assert!(d == true);
}

#[test]
fn test_core_init() {
	// FIXME: Woo... Bring out your boilerplates
	// TODO: Add assertions that callbacks are being called properly
	use self::libc::{c_int, c_uint};
	use self::ffi::glibtypes::*;
	use self::ffi::types::*;
	use self::ffi::core::*;
	use self::ffi::eventloop::*;
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

#[test]
fn test_core_ensure_single_instance() {
	assert!(core::ensure_single_instance());
}
