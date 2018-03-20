#![no_std]
#![cfg_attr(feature = "rt", feature(global_asm))]
#![cfg_attr(feature = "rt", feature(use_extern_macros))]
#![cfg_attr(feature = "rt", feature(used))]
#![feature(const_fn)]
#![allow(non_camel_case_types)]

pub extern crate mkw41z_hal as hal;
pub extern crate mkw41z;

extern crate nb;
extern crate cortex_m;
extern crate cortex_m_rt;

pub use mkw41z::*;
pub use mkw41z::interrupt::*;
pub use cortex_m_rt::*;
pub use cortex_m::*;
pub use hal::*;
