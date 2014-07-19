use libc::{c_int,c_uint, c_void, c_char};
pub type gint = c_int;
pub type gboolean = gint; // 0 == false, !false == true (..duh)
pub type guint = c_uint;
pub type gpointer = *mut c_void;
pub type GSourceFunc = fn(user_data:gpointer) -> gboolean;
pub struct GHashTable; //FIXME: investigate
