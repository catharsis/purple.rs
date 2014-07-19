#![allow(non_camel_case_types)]
#![allow(dead_code)]
use libc::c_char;
use ffi::glibtypes::*;
pub struct PurpleCore;

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
	pub fn purple_core_init(_: *const c_char) -> ();
	pub fn purple_core_quit() -> ();
	pub fn purple_core_quit_cb(unused:gpointer) -> gboolean;
	pub fn purple_core_get_ui() -> *const c_char;
	pub fn purple_get_core() -> *mut PurpleCore;
	pub fn purple_core_set_ui_ops(_: *mut PurpleCoreUiOps) -> ();
	pub fn purple_core_get_ui_ops() -> *mut PurpleCoreUiOps;
	pub fn purple_core_get_version() -> *const c_char;
	pub fn purple_core_migrate() -> gboolean;
	pub fn purple_core_ensure_single_instance() -> gboolean;
	pub fn purple_core_get_ui_info() -> *mut GHashTable;
}
