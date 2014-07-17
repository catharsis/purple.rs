#![allow(non_camel_case_types)]
#![allow(dead_code)]
extern crate libc;
use self::libc::{c_int,c_uint, c_void, c_char};

pub type PurpleRsBitmask = c_uint; //documentation type, see `bitflags!`

pub type gint = c_int;
pub type gboolean = gint; // 0 == false, !false == true (..duh)
pub type guint = c_uint;
pub type gpointer = *mut c_void;
pub type GSourceFunc = fn(user_data:gpointer) -> gboolean;
pub type PurpleInputCondition = PurpleRsBitmask;
pub type PurpleInputFunction = fn(_:gpointer, gint, PurpleInputCondition) -> ();
pub type GHashTable = (); //FIXME: investigate

pub struct PurpleEventLoopUiOps {
	/**
	 * Should create a callback timer with an interval measured in
	 * milliseconds.  The supplied @a function should be called every @a
	 * interval seconds until it returns @c FALSE, after which it should not
	 * be called again.
	 *
	 * Analogous to g_timeout_add in glib.
	 *
	 * Note: On Win32, this function may be called from a thread other than
	 * the libpurple thread.  You should make sure to detect this situation
	 * and to only call "function" from the libpurple thread.
	 *
	 * @param interval the interval in <em>milliseconds</em> between calls
	 *                 to @a function.
	 * @param data     arbitrary data to be passed to @a function at each
	 *                 call.
	 * @todo Who is responsible for freeing @a data?
	 *
	 * @return a handle for the timeout, which can be passed to
	 *         #timeout_remove.
	 *
	 * @see purple_timeout_add
	 **/
	pub timeout_add: fn(interval:guint, function:GSourceFunc, data:gpointer) -> guint,

	/**
	 * Should remove a callback timer.  Analogous to g_source_remove in glib.
	 * @param handle an identifier for a timeout, as returned by
	 *               #timeout_add.
	 * @return       @c TRUE if the timeout identified by @a handle was
	 *               found and removed.
	 * @see purple_timeout_remove
	 */
	pub timeout_remove: fn(handle:guint) -> gboolean,


	/**
	 * Should add an input handler.  Analogous to g_io_add_watch_full in
	 * glib.
	 *
	 * @param fd        a file descriptor to watch for events
	 * @param cond      a bitwise OR of events on @a fd for which @a func
	 *                  should be called.
	 * @param func      a callback to fire whenever a relevant event on @a
	 *                  fd occurs.
	 * @param user_data arbitrary data to pass to @a fd.
	 * @return          an identifier for this input handler, which can be
	 *                  passed to #input_remove.
	 *
	 * @see purple_input_add
	 */
	pub input_add: fn(fd:c_int, cond:PurpleInputCondition, func:PurpleInputFunction, user_data:gpointer) -> guint,

	/**
	 * Should remove an input handler.  Analogous to g_source_remove in glib.
	 * @param handle an identifier, as returned by #input_add.
	 * @return       @c TRUE if the input handler was found and removed.
	 * @see purple_input_remove
	 */
	pub input_remove: fn(handle:guint) -> gboolean,

	/**
	 * If implemented, should get the current error status for an input.
	 *
	 * Implementation of this UI op is optional. Implement it if the UI's
	 * sockets or event loop needs to customize determination of socket
	 * error status.  If unimplemented, <tt>getsockopt(2)</tt> will be used
	 * instead.
	 *
	 * @see purple_input_get_error
	 */
	pub input_get_error: Option<fn(fd:c_int, error:*mut c_int) -> c_int>,

	/**
	 * If implemented, should create a callback timer with an interval
	 * measured in seconds.  Analogous to g_timeout_add_seconds in glib.
	 *
	 * This allows UIs to group timers for better power efficiency.  For
	 * this reason, @a interval may be rounded by up to a second.
	 *
	 * Implementation of this UI op is optional.  If it's not implemented,
	 * calls to purple_timeout_add_seconds() will be serviced by
	 * #timeout_add.
	 *
	 * @see purple_timeout_add_seconds()
	 * @since 2.1.0
	 **/
	pub timeout_add_seconds: Option<fn(_:guint, _:GSourceFunc) -> guint>
}

pub struct PurpleCoreUiOps {
	/** Called just after the preferences subsystem is initialized; the UI
	 *  could use this callback to add some preferences it needs to be in
	 *  place when other subsystems are initialized.
	 */
	pub ui_prefs_init: Option<fn() -> ()>,

	/** Called just after the debug subsystem is initialized, but before
	 *  just about every other component's initialization.  The UI should
	 *  use this hook to call purple_debug_set_ui_ops() so that debugging
	 *  information for other components can be logged during their
	 *  initialization.
	 */
	pub debug_ui_init: Option<fn() -> ()>,

	/** Called after all of libpurple has been initialized.  The UI should
	 *  use this hook to set all other necessary UiOps structures.
	 *
	 *  @see @ref ui-ops
	 */
	pub ui_init: Option<fn() -> ()>,

	/** Called after most of libpurple has been uninitialized. */
	pub quit: Option<fn() -> ()>,

	/** Called by purple_core_get_ui_info(); should return the information
	 *  documented there.
	 */
	pub get_ui_info: Option<fn() -> GHashTable>
}
#[link(name="purple")]
extern {
	pub fn purple_debug_set_enabled(_:bool) -> ();
	pub fn purple_debug_is_enabled() -> bool;
}

#[link(name="purple")]
extern {
	pub fn purple_core_init(_: *const c_char) -> ();
	pub fn purple_core_quit() -> ();
	pub fn purple_core_quit_cb(unused:gpointer) -> gboolean;
	pub fn purple_core_get_ui() -> *const c_char;
	pub fn purple_core_set_ui_ops(_: *mut PurpleCoreUiOps) -> ();
	pub fn purple_core_get_ui_ops() -> *mut PurpleCoreUiOps;
	pub fn purple_core_get_version() -> *const c_char;
	pub fn purple_get_core() -> PurpleCore;
}

#[link(name="purple")]
extern {
	pub fn purple_eventloop_set_ui_ops(_: *mut PurpleEventLoopUiOps) -> ();
	pub fn purple_eventloop_get_ui_ops() -> *mut PurpleEventLoopUiOps;
}
