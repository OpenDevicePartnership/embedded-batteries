#![doc = include_str!("../README.md")]
#![no_std]
#![warn(missing_docs)]

/// Async Smart Battery Charger module
pub mod charger;

/// Async Smart Battery module
pub mod smart_battery;

/// Advanced Configuration and Power Interface (ACPI)
/// Power Source and Power Meter Devices module
pub use embedded_batteries::acpi;
