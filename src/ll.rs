#![allow(non_camel_case_types)]
#![allow(dead_code)]
extern crate libc;
use self::libc::{c_int,c_uint, c_void, c_char};

pub type gboolean = bool;
pub type guint = c_uint;
pub type gpointer = *mut c_void; //FIXME, (void *)
pub type GSourceFunc = fn(user_data:gpointer) -> gboolean; //FIXME, investigate
pub type PurpleInputFunction = (); //FIXME: investigate
pub type PurpleInputCondition = (); //FIXME: bitmask
pub struct PurpleEventLoopUiOps {
	pub timeout_add: fn(interval:guint, function:GSourceFunc, data:gpointer) -> c_int,
	pub timeout_remove: fn(handle:guint) -> gboolean,
	pub input_add: fn(_:c_int, _:PurpleInputCondition, _:PurpleInputFunction, _:gpointer) -> guint,
	pub input_remove: fn(_:guint) -> gboolean,
	pub input_get_error: Option<fn(_:c_int, _:*mut c_int) -> c_int>,
	pub timeout_add_seconds: Option<fn(_:guint, _:GSourceFunc) -> guint>
}

#[link(name="purple")]
extern {
	pub fn purple_debug_set_enabled(_:bool) -> ();
	pub fn purple_debug_is_enabled() -> bool;
}

#[link(name="purple")]
extern {
	pub fn purple_core_init(_: *const c_char) -> ();
	pub fn purple_core_get_ui() -> *const c_char;
}

#[link(name="purple")]
extern {
	pub fn purple_eventloop_set_ui_ops(_: *mut PurpleEventLoopUiOps) -> ();
	pub fn purple_eventloop_get_ui_ops() -> *mut PurpleEventLoopUiOps;
}
