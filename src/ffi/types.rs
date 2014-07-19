use ffi::glibtypes::{gpointer, gint};
use libc::c_uint;
pub type PurpleRsBitmask = c_uint; //documentation type, see `bitflags!`
pub type PurpleInputCondition = PurpleRsBitmask;
pub type PurpleInputFunction = fn(_:gpointer, gint, PurpleInputCondition) -> ();
