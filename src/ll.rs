#[link(name="purple")]
extern {
	pub fn purple_debug_set_enabled(_:bool) -> ();
	pub fn purple_debug_is_enabled() -> bool;
}
