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
	use self::libc::c_int;
	let e = "PURPLE_TEST";
	fn timeout_add_fn(_: guint, _: GSourceFunc, _: gpointer) -> c_int {
		println!("timeout_add_fn called");
		0
	};

	fn timeout_remove_fn(_: guint) -> gboolean {
		println!("timeout_remove_fn called");
		true
	};

	fn input_add_fn(_:c_int, _:PurpleInputCondition, _:PurpleInputFunction, _:gpointer) -> guint {
		println!("input_add_fn called");
		0
	};

	fn input_remove_fn(_:guint) -> gboolean {
		println!("input_remove called");
		true
	};

	let mut ops = PurpleEventLoopUiOps{
		timeout_add: timeout_add_fn,
		timeout_remove: timeout_remove_fn,
		input_add: input_add_fn,
		input_remove: input_remove_fn,
		input_get_error: None,
		timeout_add_seconds: None,
	};
	eventloop_set_ui_ops(&mut ops);
	core_init(e);
	assert!(e == core_get_ui().as_slice());
}
