#![crate_name = "purple"]
#![crate_type = "lib"]
#![feature(globs)]
extern crate libc;
use self::ll::*;
use std::c_str::CString;
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

pub fn core_init(ui: &str) -> () {
	unsafe {
		let cstr = ui.to_c_str();
		purple_core_init(cstr.as_ptr());
	}
}

pub fn core_get_ui() -> String {
	unsafe {
		CString::new(purple_core_get_ui(), false).as_str().unwrap().to_string()
	}
}

pub fn core_set_ui_ops(ops: *mut PurpleCoreUiOps) -> () {
	unsafe {
		purple_core_set_ui_ops(ops);
	}
}

pub fn core_get_ui_ops() -> *mut PurpleCoreUiOps {
	unsafe {
		purple_core_get_ui_ops()
	}
}
pub fn eventloop_set_ui_ops(ops: *mut PurpleEventLoopUiOps) -> () {
	unsafe {
		purple_eventloop_set_ui_ops(ops);
	}
}

pub fn eventloop_get_ui_ops() -> *mut PurpleEventLoopUiOps {
	unsafe {
		purple_eventloop_get_ui_ops()
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

#[test]
fn test_core_init() {
	// FIXME: Woo... Bring out your boilerplates
	// TODO: Add assertions that callbacks are being called properly
	use self::libc::c_int;
	let e = "PURPLE_TEST";
	fn timeout_add_fn(_: guint, _: GSourceFunc, _: gpointer) -> c_int {
		0
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
	eventloop_set_ui_ops(&mut eventloop_ops);
	core_set_ui_ops(&mut core_ops);
	core_init(e);
	assert!(e == core_get_ui().as_slice());
}
