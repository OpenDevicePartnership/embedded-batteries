use bitflags::bitflags;

/// BST: Battery Status.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BstReturn {
    /// Battery state flags indicating charging/discharging/critical status.
    pub battery_state: BatteryState,
    /// Present rate of power or current flow (in mW or mA).
    ///
    /// - `0x00000000..=0x7FFFFFFF`: Valid rate.
    /// - `0xFFFFFFFF`: Unknown rate.
    pub battery_present_rate: u32,
    /// Estimated remaining battery capacity (in mWh or mAh).
    ///
    /// - `0x00000000..=0x7FFFFFFF`: Valid capacity.
    /// - `0xFFFFFFFF`: Unknown capacity.
    pub battery_remaining_capacity: u32,
    /// Present voltage across the battery terminals (in mV).
    ///
    /// - `0x00000000..=0x7FFFFFFF`: Valid voltage.
    /// - `0xFFFFFFFF`: Unknown voltage (only for primary batteries).
    pub battery_present_voltage: u32,
}

/// Battery State (BST).
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BatteryState(u32);
bitflags! {
    impl BatteryState: u32 {
        /// Battery is discharging.
        const DISCHARGING = 1 << 0;

        /// Battery is charging.
        const CHARGING = 1 << 1;

        /// Battery is in a critical energy state.
        const CRITICAL = 1 << 2;

        /// Battery is in Battery Charge Limiting state.
        const CHARGE_LIMITING = 1 << 3;
    }
}

/// BIX: Battery Information Extended.
///
/// Represents static battery information that remains constant until the battery is replaced.
/// Supersedes `_BIF` and includes additional fields introduced in ACPI 4.0.
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BixReturn<'a> {
    /// Revision of the BIX structure. Current revision is 1.
    pub revision: u32,
    /// Unit used for capacity and rate values.
    pub power_unit: PowerUnit,
    /// Design capacity of the battery (in mWh or mAh).
    pub design_capacity: u32,
    /// Last full charge capacity (in mWh or mAh).
    pub last_full_charge_capacity: u32,
    /// Battery technology type.
    pub battery_technology: BatteryTechnology,
    /// Design voltage (in mV).
    pub design_voltage: u32,
    /// Warning capacity threshold (in mWh or mAh).
    pub design_cap_of_warning: u32,
    /// Low capacity threshold (in mWh or mAh).
    pub design_cap_of_low: u32,
    /// Number of charge/discharge cycles.
    pub cycle_count: u32,
    /// Measurement accuracy in thousandths of a percent (e.g., 80000 = 80.000%).
    pub measurement_accuracy: u32,
    /// Maximum supported sampling time (in ms).
    pub max_sampling_time: u32,
    /// Minimum supported sampling time (in ms).
    pub min_sampling_time: u32,
    /// Maximum supported averaging interval (in ms).
    pub max_averaging_interval: u32,
    /// Minimum supported averaging interval (in ms).
    pub min_averaging_interval: u32,
    /// Capacity granularity between low and warning (in mWh or mAh).
    pub battery_capacity_granularity_1: u32,
    /// Capacity granularity between warning and full (in mWh or mAh).
    pub battery_capacity_granularity_2: u32,
    /// OEM-specific model number (ASCIIZ).
    pub model_number: &'a mut [u8],
    /// OEM-specific serial number (ASCIIZ).
    pub serial_number: &'a mut [u8],
    /// OEM-specific battery type (ASCIIZ).
    pub battery_type: &'a mut [u8],
    /// OEM-specific information (ASCIIZ).
    pub oem_info: &'a mut [u8],
    /// Battery swapping capability.
    pub battery_swapping_capability: BatterySwapCapability,
}
/// Power Unit.
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PowerUnit {
    /// Capacity in mWh, rate in mW.
    MilliWatts = 0,
    /// Capacity in mAh, rate in mA.
    MilliAmps = 1,
}

impl From<PowerUnit> for u32 {
    fn from(value: PowerUnit) -> Self {
        match value {
            PowerUnit::MilliWatts => 0,
            PowerUnit::MilliAmps => 1,
        }
    }
}

/// Battery Technology.
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BatteryTechnology {
    /// Primary (non-rechargeable).
    Primary = 0,
    /// Secondary (rechargeable).
    Secondary = 1,
}

impl From<BatteryTechnology> for u32 {
    fn from(value: BatteryTechnology) -> Self {
        match value {
            BatteryTechnology::Primary => 0,
            BatteryTechnology::Secondary => 1,
        }
    }
}

/// Battery Swapping Capability.
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BatterySwapCapability {
    /// Non-swappable battery.
    NonSwappable = 0,
    /// Cold-swappable battery.
    ColdSwappable = 1,
    /// Hot-swappable battery.
    HotSwappable = 2,
}

impl From<BatterySwapCapability> for u32 {
    fn from(value: BatterySwapCapability) -> Self {
        match value {
            BatterySwapCapability::NonSwappable => 0,
            BatterySwapCapability::ColdSwappable => 1,
            BatterySwapCapability::HotSwappable => 2,
        }
    }
}

/// PSR: Power Source Status.
///
/// Represents whether a power source (e.g., AC adapter) is currently online or offline.
/// This is used to determine if the system is running on this power source.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PsrReturn {
    /// The current power source status.
    pub power_source: PowerSource,
}

/// Result of a _PSR query.
///
/// Indicates whether the power source is currently supplying power to the system
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum PowerSource {
    /// Power source is offline (not supplying power).
    Offline = 0,

    /// Power source is online (supplying power).
    Online = 1,
}

impl From<PowerSource> for u32 {
    fn from(value: PowerSource) -> Self {
        match value {
            PowerSource::Offline => 0,
            PowerSource::Online => 1,
        }
    }
}

/// PIF: Power Source Information.
///
/// Represents static information about a power source device. This information
/// remains constant until the power source is changed.
#[derive(PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Pif<'a> {
    /// Bitfield describing the state and characteristics of the power source.
    pub power_source_state: PowerSourceState,
    /// Maximum rated output power in milliwatts (mW).
    ///
    /// 0xFFFFFFFF indicates the value is unavailable.
    pub max_output_power: u32,
    /// Maximum rated input power in milliwatts (mW).
    ///
    /// 0xFFFFFFFF indicates the value is unavailable.
    pub max_input_power: u32,
    /// OEM-specific model number (ASCIIZ). Empty string if not supported.
    pub model_number: &'a mut [u8],
    /// OEM-specific serial number (ASCIIZ). Empty string if not supported.
    pub serial_number: &'a mut [u8],
    /// OEM-specific information (ASCIIZ). Empty string if not supported.
    pub oem_info: &'a mut [u8],
}

/// Power Source State.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PowerSourceState(u32);
bitflags! {
    impl PowerSourceState: u32 {
        /// Indicates the power source is redundant.
        const REDUNDANT = 1 << 0;

        /// Indicates the power source is shared across multiple machines.
        const SHARED = 1 << 1;
    }
}

/// BPS: Battery Power Source Information.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Bps {
    /// Current revision of the BPS structure.
    ///
    /// The current revision is 1.
    pub revision: u32,

    /// Instantaneous Peak Power Level in mW or mA.
    ///
    /// Represents the instantaneous peak output power of the battery, based on the Power Unit
    /// value returned by `_BIX`. The time period is specified in the `instantaneous_peak_power_period`.
    /// This value accounts for battery resistances and the minimum system voltage.
    /// If unsupported, this field should be zero.
    pub instantaneous_peak_power_level: u32,

    /// Instantaneous Peak Power Period in milliseconds.
    ///
    /// The duration for which the battery can supply the `instantaneous_peak_power_level`.
    /// If unsupported, this field should be zero.
    pub instantaneous_peak_power_period: u32,

    /// Sustainable Peak Power Level in mW or mA.
    ///
    /// Represents the sustainable peak output power of the battery, based on the Power Unit
    /// value returned by `_BIX`. The time period is specified in the `sustainable_peak_power_period`.
    /// This value accounts for battery resistances and the minimum system voltage.
    /// If unsupported, this field should be zero.
    pub sustainable_peak_power_level: u32,

    /// Sustainable Peak Power Period in milliseconds.
    ///
    /// The duration for which the battery can supply the `sustainable_peak_power_level`.
    /// If unsupported, this field should be zero.
    pub sustainable_peak_power_period: u32,
}

/// BTP: Battery Trip Point.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Btp {
    /// 0 - Clear the trip point.
    /// 1 - 0x7FFFFFFF - New trip point, in units of mWh or mAh depending on the Power Units value
    pub trip_point: u32,
}

/// BPT: Battery Power Threshold Configuration.
///
/// Represents a request to set or clear battery power delivery capability thresholds.
/// Used by the OS Power Management (OSPM) to configure notifications for changes
/// in battery power delivery capabilities.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Bpt {
    /// Revision of the BPT structure.
    ///
    /// For this version of the specification, the revision must be set to 1.
    pub revision: u32,

    /// Type of threshold to set or clear.
    pub threshold_id: ThresholdId,

    /// Threshold value in mW or mA.
    ///
    /// This value is based on the Power Unit field returned by `_BIX`.
    /// A value of `0` disables the selected threshold.
    /// The value must not exceed the maximum values reported by `_BPC`.
    pub threshold_value: u32,
}

/// Enum representing the threshold type for battery power delivery capability.
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ThresholdId {
    /// Clear all threshold trip points.
    ClearAll = 0,

    /// Set Instantaneous Peak Power Threshold.
    InstantaneousPeakPower = 1,

    /// Set Sustainable Peak Power Threshold.
    SustainablePeakPower = 2,
}

/// Return codes for BPT operations.
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BptReturnStatus {
    /// Operation completed successfully.
    Success = 0x00000000,

    /// Failure due to an invalid threshold value.
    InvalidThresholdValue = 0x00000001,

    /// Failure due to hardware timeout.
    HardwareTimeout = 0x00000002,

    /// Failure due to an unknown hardware error.
    UnknownHardwareError = 0x00000003,

    /// Failure due to unsupported threshold type.
    UnsupportedThresholdType = 0x00000004,

    /// Failure due to unsupported revision.
    UnsupportedRevision = 0x00000005,
}

/// BPC: Battery Power Characteristics.
///
/// Represents static values returned by the platform firmware that describe
/// the battery's power delivery capabilities and threshold support.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Bpc {
    /// Revision of the BPC structure.
    ///
    /// For this version of the specification, the revision must be set to 1.
    pub revision: u32,

    /// Power threshold support capability of the platform firmware.
    ///
    /// This is a bitfield indicating which types of power thresholds are supported.
    pub power_threshold_support: PowerThresholdSupport,

    /// Maximum supported threshold for instantaneous peak power (in mW or mA).
    ///
    /// This value defines the upper bound for the instantaneous peak power threshold
    /// that can be set using `_BPT`.
    pub max_instantaneous_peak_power_threshold: u32,

    /// Maximum supported threshold for sustainable peak power (in mW or mA).
    ///
    /// This value defines the upper bound for the sustainable peak power threshold
    /// that can be set using `_BPT`.
    pub max_sustainable_peak_power_threshold: u32,
}

/// Bitflags representing the power threshold support capabilities of the platform firmware.
///
/// These values are encoded in the lower two bits of the `Power Threshold Support` field.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct PowerThresholdSupport(u32);
bitflags! {
    impl PowerThresholdSupport: u32 {
        /// Supports Instantaneous Peak Power Threshold.
        const INSTANTANEOUS = 1 << 0;
        /// Supports Sustainable Peak Power Threshold.
        const SUSTAINABLE = 1 << 1;
    }
}

/// BMC: Batery Maintenance Control
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Bmc {
    /// Feature control flags used to configure battery maintenance behavior.
    pub maintenance_control_flags: BmcControlFlags,
}

/// Bitflags representing the power threshold support capabilities of the platform firmware.
///
/// These values are encoded in the lower two bits of the `Power Threshold Support` field.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BmcControlFlags(u32);
bitflags! {
    impl BmcControlFlags: u32 {
        /// Set to initiate an AML-controlled calibration cycle. Clear to end it.
        const CALIBRATION_CYCLE = 1 << 0;

        /// Set to disable charging. Clear to enable charging.
        const DISABLE_CHARGING = 1 << 1;

        /// Set to allow discharging while AC power is available.
        const ALLOW_DISCHARGE_ON_AC = 1 << 2;

        /// Set to request suspension of Battery Charge Limiting mode.
        const SUSPEND_CHARGE_LIMITING = 1 << 3;
    }
}

/// BMD: Battery Maintenance Data.
///
/// Contains information about the battery’s capabilities and current state
/// related to calibration and charger control features.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Bmd {
    /// Current status flags indicating battery maintenance state.
    pub status_flags: BmdStatusFlags,

    /// Capability flags indicating supported battery maintenance features.
    pub capability_flags: BmdCapabilityFlags,

    /// Recommended recalibration count.
    ///
    /// - `0x00000000`: Only calibrate when Status Flag bit [3] is set.
    /// - `0x00000001..=0xFFFFFFFF`: Calibrate after this many battery cycles.
    pub recalibrate_count: u32,

    /// Estimated time (in seconds) to recalibrate the battery if the system enters standby.
    ///
    /// - `0x00000000`: Standby not supported.
    /// - `0x00000001..=0xFFFFFFFE`: Estimated time in seconds.
    /// - `0xFFFFFFFF`: Time unknown.
    pub quick_recalibrate_time: u32,

    /// Estimated time (in seconds) to recalibrate the battery without standby.
    ///
    /// - `0x00000000`: Calibration may not be successful.
    /// - `0x00000001..=0xFFFFFFFE`: Estimated time in seconds.
    /// - `0xFFFFFFFF`: Time unknown.
    pub slow_recalibrate_time: u32,
}

/// Status Flags returned by _BMD.
///
/// These indicate the current state of battery maintenance operations.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BmdStatusFlags(u32);
bitflags! {
    impl BmdStatusFlags: u32 {
        /// Battery is running an AML-controlled calibration cycle.
        const AML_CALIBRATION_ACTIVE = 1 << 0;

        /// Charging has been disabled.
        const CHARGING_DISABLED = 1 << 1;

        /// Battery is allowed to discharge while AC is available.
        const DISCHARGE_ON_AC = 1 << 2;

        /// Battery should be recalibrated.
        const RECALIBRATION_NEEDED = 1 << 3;

        /// OS should enter standby to speed up calibration.
        const STANDBY_RECOMMENDED = 1 << 4;

        /// Battery Charge Limiting cannot be suspended due to thermal conditions.
        const CHARGE_LIMIT_THERMAL_LOCK = 1 << 5;

        /// Battery Charge Limiting cannot be suspended for protection reasons.
        const CHARGE_LIMIT_PROTECTION_LOCK = 1 << 6;
    }
}

/// Capability Flags returned by _BMD.
///
/// These indicate which battery maintenance features are supported.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct BmdCapabilityFlags(u32);
bitflags! {
    impl BmdCapabilityFlags: u32 {
        /// AML-controlled calibration cycle is supported.
        const AML_CALIBRATION_SUPPORTED = 1 << 0;

        /// Disabling the charger is supported.
        const CHARGER_DISABLE_SUPPORTED = 1 << 1;

        /// Discharging while on AC is supported.
        const DISCHARGE_ON_AC_SUPPORTED = 1 << 2;

        /// _BMC affects all batteries in the system.
        const GLOBAL_CONTROL = 1 << 3;

        /// Calibration must start with a full charge.
        const FULL_CHARGE_BEFORE_CALIBRATION = 1 << 4;

        /// Battery Charge Limiting suspension is supported.
        const CHARGE_LIMIT_SUSPEND_SUPPORTED = 1 << 5;
    }
}

/// BCT: Battery Charge Time.
///
/// Represents a request to estimate the time required to charge the battery
/// to a specified percentage of its Last Full Charge Capacity.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Bct {
    /// Target charge level as a percentage of Last Full Charge Capacity (1–100).
    ///
    /// For example, `96` means 96% of full charge.
    pub charge_level_percent: u32,
}

/// Result of a _BCT query.
///
/// This enum represents the possible return values from the `_BCT` method.
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BctReturnResult {
    /// The requested charge level is invalid (less than current or greater than 100%).
    InvalidTarget = 0x00000000,

    /// Estimated time in seconds to reach the target charge level.
    EstimatedTime(u32),

    /// Charging time is unknown.
    Unknown = 0xFFFFFFFF,
}
impl From<u32> for BctReturnResult {
    fn from(value: u32) -> Self {
        match value {
            0x00000000 => BctReturnResult::InvalidTarget,
            0xFFFFFFFF => BctReturnResult::Unknown,
            seconds => BctReturnResult::EstimatedTime(seconds),
        }
    }
}

/// BTM: Battery Time.
///
/// Represents a request to estimate the remaining runtime of the battery
/// while it is discharging at a specified rate.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Btm {
    /// Discharge rate in mA or mW.
    ///
    /// - `0`: Use the current average discharge rate.
    /// - `1..=0x7FFFFFFF`: Specific discharge rate to evaluate.
    pub discharge_rate: u32,
}

/// Result of a _BTM query.
///
/// This enum represents the possible return values from the `_BTM` method.
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BtmReturnResult {
    /// The discharge rate is too high, or the battery is critical (if input was 0).
    RateTooHighOrBatteryCritical = 0x00000000,

    /// Estimated runtime in seconds.
    EstimatedRuntime(u32),

    /// Runtime is unknown.
    Unknown = 0xFFFFFFFF,
}

impl From<u32> for BtmReturnResult {
    fn from(value: u32) -> Self {
        match value {
            0x00000000 => BtmReturnResult::RateTooHighOrBatteryCritical,
            0xFFFFFFFF => BtmReturnResult::Unknown,
            seconds => BtmReturnResult::EstimatedRuntime(seconds),
        }
    }
}

/// BMS: Battery Measurement Sampling Time.
///
/// Used to set the sampling interval (in milliseconds) for battery capacity measurements
/// such as present rate and remaining capacity reported by `_BST`.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Bms {
    /// Desired sampling time in milliseconds.
    ///
    /// Valid range: `0x00000001` to `0xFFFFFFFF`.
    pub sampling_time_ms: u32,
}

/// Result of a _BMS operation.
///
/// Represents the possible return values from the `_BMS` method.
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BmsReturnResult {
    /// Sampling time was successfully set.
    Success = 0,

    /// Sampling time is outside the battery's supported range.
    OutOfRange = 1,
}

impl From<BmsReturnResult> for u32 {
    fn from(value: BmsReturnResult) -> Self {
        match value {
            BmsReturnResult::Success => 0,
            BmsReturnResult::OutOfRange => 1,
        }
    }
}

/// BMA: Battery Measurement Averaging Interval.
///
/// Used to set the averaging interval (in milliseconds) for battery capacity measurements
/// such as remaining capacity and present rate reported by `_BST`.
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Bma {
    /// Desired averaging interval in milliseconds.
    ///
    /// Valid range: `0x00000001` to `0xFFFFFFFF`.
    pub averaging_interval_ms: u32,
}

/// Result of a _BMA operation.
///
/// Represents the possible return values from the `_BMA` method.
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum BmaReturnResult {
    /// Averaging interval was successfully set.
    Success = 0,

    /// Averaging interval is outside the battery's supported range.
    OutOfRange = 1,
}

impl From<BmaReturnResult> for u32 {
    fn from(value: BmaReturnResult) -> Self {
        match value {
            BmaReturnResult::Success => 0,
            BmaReturnResult::OutOfRange => 1,
        }
    }
}
