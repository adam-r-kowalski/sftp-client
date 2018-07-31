#![feature(unboxed_closures, fn_traits,extern_prelude)]

extern crate ssh2;
extern crate rpassword;

pub mod connection;
pub mod input;
pub mod local;
pub mod logger;
pub mod menu;
pub mod remote;
