#![feature(pub_restricted)]
extern crate libc;
#[macro_use]
extern crate log;
extern crate env_logger;

pub mod loader_interface;
mod ffi_types;
mod entrypoint;
mod extension;
mod dispatch;
mod version;
mod physical_device;
mod device;
//mod mem;


use std::sync::{Once, ONCE_INIT};

// Init logging
pub static LOG: Once = ONCE_INIT;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
