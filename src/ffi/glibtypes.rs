use libc::{c_int,c_uint, c_void, c_char};
pub type gint = c_int;
pub type gboolean = gint; // 0 == false, !false == true (..duh)
pub type guint = c_uint;
pub type gpointer = *mut c_void;
pub type GSourceFunc = fn(user_data:gpointer) -> gboolean;
pub struct GHashTable; //FIXME: investigate

pub trait GbooleanCastable {
	fn as_gboolean(&self) -> gboolean;
}

pub trait BoolCastable {
	fn as_bool(&self) -> bool;
}

impl GbooleanCastable for bool {
	fn as_gboolean(&self) -> gboolean {
		match *self {
			false => 0,
			true => 1
		}
	}
}

impl BoolCastable for gboolean {
	fn as_bool(&self) -> bool {
		0 != *self
	}
}

