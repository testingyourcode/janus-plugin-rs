#![allow(non_camel_case_types)]
#![deny(missing_debug_implementations)]

extern crate glib_sys;
extern crate jansson_sys;
use std::os::raw::{c_char, c_int};

pub mod plugin;
pub mod events;
pub mod rtcp;
pub mod sdp;

#[cfg(feature="refcount")]
#[repr(C)]
#[derive(Debug)]
pub struct janus_refcount {
    pub count: c_int,
    pub free: extern "C" fn(obj: *const janus_refcount),
}

extern "C" {
    pub static janus_log_timestamps: c_int;
    pub static janus_log_colors: c_int;
    pub static janus_log_level: c_int;

    pub fn janus_get_api_error(error: c_int) -> *const c_char;
    pub fn janus_vprintf(format: *const c_char, ...);

    #[cfg(feature="refcount")]
    pub static refcount_debug: c_int;
}
