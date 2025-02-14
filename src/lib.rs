//! A rust interface to the x2apic interrupt architecture.

#![no_std]
#![allow(internal_features)]
#![feature(ptr_internals, negative_impls)]
#![deny(missing_docs)]

pub mod ioapic;
pub mod lapic;
