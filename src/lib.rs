//! ckb-std
//!
//! # Modules
//!
//! * `high_level` module: defines high level syscall API
//! * `syscalls` module: defines low level [CKB syscalls](https://github.com/nervosnetwork/rfcs/blob/master/rfcs/0009-vm-syscalls/0009-vm-syscalls.md)
//! * `debug!` macro: a `println!` like macro helps debugging
//! * `entry!` macro: defines contract entry point
//! * `default_alloc!` and `libc_alloc!` macro: defines global allocator for no-std rust

#![no_std]
#![feature(llvm_asm)]

extern crate alloc;

pub mod ckb_constants;
pub mod debug;
pub mod entry;
pub mod error;
pub mod global_alloc;
#[cfg(feature = "libc")]
pub mod libc_alloc_wrap;
pub mod since;
pub mod syscalls;
#[cfg(feature = "buddy-alloc")]
pub use buddy_alloc;
#[cfg(feature = "ckb-types")]
pub mod high_level;
#[cfg(feature = "ckb-types")]
pub use ckb_types;
