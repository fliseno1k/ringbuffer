#![cfg_attr(not(feature = "std"), no_std)]
#![allow(dead_code)]
#![allow(unused_imports)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
pub(self) use alloc::vec::Vec;
#[cfg(feature = "std")]
pub(self) use std::vec::Vec;

#[cfg(not(feature = "std"))]
pub(self) use alloc::boxed::Box;
#[cfg(feature = "std")]
pub(self) use std::boxed::Box;

mod traits;

pub mod ringbuffer_fixed;
