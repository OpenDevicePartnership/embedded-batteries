#![doc = include_str!("../README.md")]
#![no_std]
#![warn(missing_docs)]

// Shared types

/// Charging current is measured in milliamps, where 1mA is 1
pub type MilliAmps = u16;

/// Charging voltage is measured in millivolts, where 1mV is 1
pub type MilliVolts = u16;

/// Charging current is measured in milliamps, where 1mA is 1
pub type MilliAmpsSigned = i16;

/// Charging voltage is measured in millivolts, where 1mV is 1
pub type MilliVoltsSigned = i16;

/// Blocking Smart Battery Charger module
pub mod charger;

/// Blocking Smart Battery module
pub mod smart_battery;

/// Advanced Configuration and Power Interface (ACPI)
/// Power Source and Power Meter Devices module
pub mod acpi;
