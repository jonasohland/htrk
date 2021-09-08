use htrk;
use std::ffi::c_void;
use std::os::raw::c_char;

mod c;

/// Callback function pointer type
#[allow(non_camel_case_types)]
type htrk_discovery_callback = extern fn(*const c_char, *mut c_void) -> c_void;

/// Opaque handle to a discovery context
#[allow(non_camel_case_types)]
type htrk_discovery_ctx = c_void;

#[no_mangle]
pub extern "C" fn htrk_discovery_new(_ctx: *mut *mut htrk_discovery_ctx) {
    let ctx = htrk::discovery::DiscoveryContext::new();
}

#[no_mangle]
pub extern "C" fn htrk_discovery_free(_ctx: *mut htrk_discovery_ctx) {}

#[no_mangle]
pub extern "C" fn htrk_discovery_start(
    _ctx: *mut htrk_discovery_ctx,
    _user_data: *mut c_void,
    _cb: htrk_discovery_callback) {}

#[no_mangle]
pub extern "C" fn htrk_discovery_stop(_ctx: *mut htrk_discovery_ctx) {}