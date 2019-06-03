#![feature(asm, concat_idents, const_fn, try_from, used)]
#![feature(exclusive_range_pattern)]
#![no_std]
#![crate_name = "e310x"]
#![crate_type = "rlib"]

mod interrupts;

pub mod chip;
pub mod gpio;
pub mod prci;
pub mod pwm;
pub mod rtc;
pub mod uart;
pub mod watchdog;
