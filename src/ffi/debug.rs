#![allow(non_camel_case_types)]
use ffi::glibtypes::*;
use libc::c_char;

#[repr(C)]
pub enum PurpleDebugLevel {
	PURPLE_DEBUG_ALL = 0,
	PURPLE_DEBUG_MISC,
	PURPLE_DEBUG_INFO,
	PURPLE_DEBUG_WARNING,
	PURPLE_DEBUG_ERROR,
	PURPLE_DEBUG_FATAL
}

pub struct PurpleDebugUiOps
{
	pub print: fn(level:PurpleDebugLevel, category: *const c_char, arg_s: *const c_char) -> (),
	pub is_enabled:  fn(level:PurpleDebugLevel, category: *const c_char) -> gboolean
}

#[link(name="purple")]
extern {
	pub fn purple_debug_set_enabled(_:gboolean) -> ();
	pub fn purple_debug_is_enabled() -> gboolean;
	pub fn purple_debug_set_verbose(verbose:gboolean) -> ();
	pub fn purple_debug_is_verbose() -> gboolean;
	pub fn purple_debug(level:PurpleDebugLevel, category:*const c_char,
			format:*const c_char, ...) -> ();
	pub fn purple_debug_misc(category: *const c_char, format:  *const c_char, ...) -> ();
	pub fn purple_debug_info(category: *const c_char, format:  *const c_char, ...) -> ();
	pub fn purple_debug_warning(category: *const c_char, format:  *const c_char, ...) -> ();
	pub fn purple_debug_error(category: *const c_char, format:  *const c_char, ...) -> ();
	pub fn purple_debug_fatal(category: *const c_char, format:  *const c_char, ...) -> ();
	pub fn purple_debug_init() -> ();
}




