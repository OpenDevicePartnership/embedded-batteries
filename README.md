# `embedded-batteries`
[![LICENSE](https://img.shields.io/badge/License-MIT-blue)](LICENSE)

## Introduction
This is a Hardware Abstraction Layer (HAL) for battery fuel gauges and battery chargers used in embedded systems, with the goal of being hardware and platform independent.

Specifically, traits are defined for both battery fuel gauges and battery chargers with functionality that conforms to the [Smart Battery System v1.1 (SBS) specification](https://sbs-forum.org/specs/sbdat110.pdf).

Drivers for fuel gauges and charging controllers should implement these traits to provide a standard way of interfacing with the device.

Additionally, the [ACPI Specification's Power Source and Power Meter Devices](https://uefi.org/htmlspecs/ACPI_Spec_6_4_html/10_Power_Source_and_Power_Meter_Devices/Power_Source_and_Power_Meter_Devices.html) battery control messages are defined as an supplementary superset of the Smart Battery specification, for users building systems compliant with ACPI.

## Crates

| Crate | Description |
|-|-|
| [embedded-batteries](./embedded-batteries) | Core traits, ACPI battery control methods, blocking version |
| [embedded-batteries-async](./embedded-batteries-async) | Core traits, ACPI battery control methods, async version |

## MSRV

Currently, rust `1.83` and up is supported.

## License

Licensed under the terms of the [MIT license](http://opensource.org/licenses/MIT).

## Contribution

Unless you explicitly state otherwise, any contribution submitted for
inclusion in the work by you shall be licensed under the terms of the
MIT license.

License: MIT
