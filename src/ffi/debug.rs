use ffi::glibtypes::*;
use libc::c_char;
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
	pub print: fn(level:PurpleDebugLevel, category: *const c_char, arg_s: *const char) -> (),
	pub is_enabled:  fn(level:PurpleDebugLevel, category: *const c_char) -> gboolean
}

#[link(name="purple")]
extern {
	pub fn purple_debug_set_enabled(_:gboolean) -> ();
	pub fn purple_debug_is_enabled() -> gboolean;
	pub fn purple_debug_set_verbose(verbose:gboolean) -> ();
	pub fn purple_debug_is_verbose() -> gboolean;
}




