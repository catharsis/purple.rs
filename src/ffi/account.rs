#![allow(non_camel_case_types)]
use ffi::glibtypes::*;
use libc::{c_char, c_void};

#[repr(C)]
pub enum PurpleAccountRequestType {
	PURPLE_ACCOUNT_REQUEST_AUTHORIZATION = 0, /* Account authorization request */
	_PURPLE_ACCOUNT_REQUEST_UNUSED = 999 /* Avoid repr(C) univariants: https://github.com/rust-lang/rust/issues/10292 */
}

#[repr(C)]
pub enum PurpleAccountRequestResponse {
	PURPLE_ACCOUNT_RESPONSE_IGNORE = -2,
	PURPLE_ACCOUNT_RESPONSE_DENY = -1,
	PURPLE_ACCOUNT_RESPONSE_PASS = 0,
	PURPLE_ACCOUNT_RESPONSE_ACCEPT = 1
}


pub struct _PurpleAccountUiOps;
pub struct _PurpleAccount;

pub type PurpleAccountUiOps = _PurpleAccountUiOps;
pub type PurpleAccount = _PurpleAccount;

pub type PurpleFilterAccountFunc = fn (account: *mut PurpleAccount) -> gboolean;
pub type PurpleAccountRequestAuthorizationCb = fn (_: *mut c_void) -> ();
pub type PurpleAccountRegistrationCb = fn (account: *mut PurpleAccount, succeeded: gboolean, user_data: *mut c_void) -> ();
pub type PurpleAccountUnregistrationCb = fn (account: *mut PurpleAccount, succeeded: gboolean, user_data: *mut c_void) -> ();
pub type PurpleSetPublicAliasSuccessCallback = fn (account: *mut PurpleAccount, new_alias: *const c_char) -> ();
pub type PurpleSetPublicAliasFailureCallback = fn (account: *mut PurpleAccount, error: *const c_char) -> ();
pub type PurpleGetPublicAliasSuccessCallback = fn (account: *mut PurpleAccount, alias: *const c_char) -> ();
pub type PurpleGetPublicAliasFailureCallback = fn (account: *mut PurpleAccount, error: *const c_char) -> ();


#[link(name="purple")]
extern {
	pub fn purple_account_new(username: *const c_char, protocol_id: *const c_char) -> *mut PurpleAccount;
}
