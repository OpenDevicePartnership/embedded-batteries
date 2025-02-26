use bitfield_struct::bitfield;

use crate::{MilliAmpsSigned, MilliVolts};

/// Smart Battery error.
pub trait Error: core::fmt::Debug {
    /// Convert error to a generic Smart Battery error kind.
    ///
    /// By using this method, Smart Battery errors freely defined by HAL implementations
    /// can be converted to a set of generic Smart Battery errors upon which generic
    /// code can act.
    fn kind(&self) -> ErrorKind;
}

impl Error for core::convert::Infallible {
    #[inline]
    fn kind(&self) -> ErrorKind {
        match *self {}
    }
}

/// Smart Battery error kind.
///
/// This represents a common set of Smart Battery operation errors. HAL implementations are
/// free to define more specific or additional error types. However, by providing
/// a mapping to these common Smart Battery errors, generic code can still react to them.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[non_exhaustive]
pub enum ErrorKind {
    /// An error occurred on the underlying peripheral supporting the sensor.
    /// e.g. An I2C bus error occurs for an I2C enabled Smart Battery.
    /// The original error may contain more information.
    CommError,
    /// An error occured and was reported by a read from the BatteryStatus (0x16) register.
    BatteryStatus(ErrorCode),
    /// A different error occurred. The original error may contain more information.
    Other,
}

impl Error for ErrorKind {
    #[inline]
    fn kind(&self) -> ErrorKind {
        *self
    }
}

impl core::fmt::Display for ErrorKind {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::CommError => write!(f, "Error communicating with Smart Battery"),
            Self::BatteryStatus(_) => write!(
                f,
                "Error reported by BatteryService (0x16) register. The original error may contain more information"
            ),
            Self::Other => write!(
                f,
                "A different error occurred. The original error may contain more information"
            ),
        }
    }
}

/// Smart Battery error type trait.
///
/// This just defines the error type, to be used by the other Smart Battery traits.
pub trait ErrorType {
    /// Error type.
    type Error: Error;
}

impl<T: ErrorType + ?Sized> ErrorType for &mut T {
    type Error = T::Error;
}

/// Depending on the value of the CapacityMode bit, the Smart Battery will use milliamps or centiwatts.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CapacityModeValue {
    /// Unsigned Milliamp or MilliampHour representation, used when CapacityMode bit = 0.
    MilliAmpUnsigned(u16),
    /// Unsigned Centiwatt or CentiwattHour representation, used when CapacityMode bit = 1.
    CentiWattUnsigned(u16),
}

/// Time is measured in minutes, where 1 minute is 1
pub type Minutes = u16;

/// Depending on the value of the CapacityMode bit, the Smart Battery will use milliamps or centiwatts.
/// Signed to represent negative currents and capacities.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum CapacityModeSignedValue {
    /// Signed Milliamp or MilliampHour representation, used when CapacityMode bit = 0.
    MilliAmpSigned(i16),
    /// Signed Milliamp or MilliampHour representation, used when CapacityMode bit = 1.
    CentiWattSigned(i16),
}

/// Temperature is measured in decikelvins, where 0.1 Kelvin is 1.
pub type DeciKelvin = u16;

/// Percent, 1% is 1.
pub type Percent = u8;

/// Cycles, 1 cycle is 1.
pub type Cycles = u16;

/// Error codes that must be supported by the Smart Battery.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum ErrorCode {
    /// The Smart Battery processed the function code
    /// without detecting any errors.
    Ok = 0,

    /// The Smart Battery is unable to process the function
    /// code at this time.
    Busy = 1,

    /// The Smart Battery detected an attempt to read or
    /// write to a function code reserved by this version of
    /// the specification. The Smart Battery detected an
    /// attempt to access an unsupported optional
    /// manufacturer function code.
    ReservedCmd = 2,

    /// The Smart Battery does not support this function
    /// code which is defined in this version of the
    /// specification.
    UnsupportedCmd = 3,

    /// The Smart Battery detected an attempt to write to a
    /// read only function code.
    AccessDenied = 4,

    /// The Smart Battery detected a data overflow or
    /// under flow.
    UnderOverFlow = 5,

    /// The Smart Battery detected an attempt to write to a
    /// function code with an incorrect size data block.
    BadSize = 6,

    /// The Smart Battery detected an unidentifiable error.
    UnknownError = 7,
}

impl ErrorCode {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            0 => Self::Ok,
            1 => Self::Busy,
            2 => Self::ReservedCmd,
            3 => Self::UnsupportedCmd,
            4 => Self::AccessDenied,
            5 => Self::UnderOverFlow,
            6 => Self::BadSize,
            _ => Self::UnknownError,
        }
    }
}

impl From<u8> for ErrorCode {
    fn from(value: u8) -> Self {
        Self::from_bits(value)
    }
}

/// Revision of SBS Spec, used in specification_info().
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Revision {
    /// Version 1.0 and 1.1.
    Version1And1Dot1 = 1,
}

impl Revision {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            1 => Self::Version1And1Dot1,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<u8> for Revision {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Version1And1Dot1),
            _ => Err(()),
        }
    }
}

/// Version of SBS Spec, used in specification_info().
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[repr(u8)]
pub enum Version {
    /// Reserved.
    Reserved = 0,

    /// Version 1.0.
    Version1 = 1,

    /// Version 1.1.
    Version1Dot1 = 2,

    /// Version 1.1 with optional PEC support.
    Version1Dot1Pec = 3,
}

impl Version {
    const fn into_bits(self) -> u8 {
        self as _
    }

    const fn from_bits(value: u8) -> Self {
        match value {
            1 => Self::Version1,
            2 => Self::Version1Dot1,
            3 => Self::Version1Dot1Pec,
            _ => Self::Reserved,
        }
    }
}

impl From<u8> for Version {
    fn from(value: u8) -> Self {
        Self::from_bits(value)
    }
}

/// Return value of the manufacture_date() function (0x1b). The date is packed in the
/// following fashion: (year-1980) * 512 + month * 32 + day.
#[bitfield(u16, defmt = cfg(feature = "defmt"))]
pub struct ManufactureDate {
    /// 1 - 31 (corresponds to date).
    #[bits(5)]
    pub day: usize,

    /// 1 - 12 (corresponds to month number).
    #[bits(4)]
    pub month: usize,

    /// 0 - 127 (corresponds to year biased by 1980).
    /// Add 1980 to the year to get the true year.
    #[bits(7)]
    pub year: usize,
}

/// Return value of the battery_mode() function (0x03). See the SBS spec for more information.
#[bitfield(u16, defmt = cfg(feature = "defmt"))]
pub struct BatteryModeFields {
    /// INTERNAL_CHARGE_CONTROLLER bit set indicates that the battery pack contains its own internal
    /// charge controller. When the bit is set, this optional function is supported and the
    /// CHARGE_CONTROLLER_ENABLED bit will be available for activation and control of the actual
    /// internal charger.
    #[bits(1, access = RO)]
    pub internal_charge_controller: bool,

    /// PRIMARY_BATTERY_SUPPORT bit set indicates that the battery pack has the ability to act as either
    /// the primary or secondary battery in a system. When the bit is set, this function is supported and the
    /// PRIMARY_BATTERY bit will be available for activation and control of this function
    #[bits(1, access = RO)]
    pub primary_battery_support: bool,

    #[bits(5)]
    __: u8,

    /// CONDITION_FLAG bit set indicates that the battery is requesting a conditioning cycle. A conditioning
    /// cycle may be requested because of the characteristics of the battery chemistry and/or the electronics in
    /// combination with the usage pattern.
    #[bits(1, access = RO)]
    pub condition_flag: bool,

    /// CHARGE_CONTROLLER_ENABLED bit is set to enable the battery pack’s internal charge controller.
    /// When this bit is cleared, the internal charge controller is disabled (default). This bit is active only when the
    /// INTERNAL_CHARGE_CONTROLLER bit is set, indicating that this function is supported. The status of
    /// a battery pack’s internal charge controller can be determined by reading this bit
    pub charge_controller_enabled: bool,

    /// PRIMARY_BATTERY bit is set to enable a battery to operate as the primary battery in a system. When
    /// this bit is cleared, the battery operates in a secondary role (default). This bit is active only when the
    /// PRIMARY_BATTERY_SUPPORT bit is set. The role that the battery is playing can be determined by
    /// reading this bit.
    pub primary_battery: bool,

    #[bits(3)]
    __: u8,

    /// ALARM_MODE bit is set to disable the Smart Battery's ability to master the SMBus and send
    /// AlarmWarning() messages to the SMBus Host and the Smart Battery Charger. When set, the Smart Battery
    /// will NOT master the SMBus and AlarmWarning() messages will NOT be sent to the SMBus Host and the
    /// Smart Battery Charger for a period of no more than 65 seconds and no less than 45 seconds. When
    /// cleared (default), the Smart Battery WILL send the AlarmWarning() messages to the SMBus Host and the
    /// Smart Battery Charger any time an alarm condition is detected. (See also Section 5.4 of the SBS spec
    /// for a more detailed explanation of alarm conditions and operations.)
    ///
    /// When the ALARM_MODE bit is set, the system assumes responsibility for detecting and
    /// responding to Smart Battery alarms by reading the BatteryStatus() to determine if any of the alarm
    /// bit flags are set. At a minimum, this requires the system to poll the Smart Battery BatteryStatus()
    /// every 10 seconds at all times the SMBus is active. The system is expected to take appropriate
    /// action.
    ///
    /// The ALARM_MODE bit is automatically cleared by the Smart Battery electronics every 60
    /// seconds so that any accidental activation of this mode will not be persistent. A SMBus Host which
    /// does not want the Smart Battery to be a master on the SMBus must therefore continually set this
    /// bit at least once per 45 seconds to keep the ALARM_MODE bit set
    pub alarm_mode: bool,

    /// CHARGER_MODE bit enables or disables the Smart Battery's transmission of ChargingCurrent() and
    /// ChargingVoltage() messages to the Smart Battery Charger. When set, the Smart Battery will NOT transmit
    /// ChargingCurrent() and ChargingVoltage() values to the Smart Battery Charger. When cleared, the Smart
    /// Battery will transmit the ChargingCurrent() and ChargingVoltage() values to the Smart Battery Charger
    /// when charging is desired. (See Section 5.3 of the SBS spec for a more detailed explanation.)
    ///
    /// When the CHARGER_MODE bit is set, the system assumes responsibility for safely charging the
    /// Smart Battery. At a minimum, this requires the system to poll the Smart Battery for
    /// ChargingVoltage() and ChargingCurrent() at the same rate the Smart Battery would normally send
    /// these charging messages to the Smart Battery Charger (e.g. every 5 seconds to 60 seconds.)
    /// The CHARGER_MODE bit allows a SMBus Host or Smart Battery Charger to disable the Smart
    /// Battery's broadcast of the ChargingCurrent() and ChargingVoltage().
    ///
    /// The use of CHARGER_MODE does NOT affect the use of ALARM_MODE. If only
    /// CHARGER_MODE bit is set, AlarmWarning messages relating to charging will still occur and be
    /// broadcast the Smart Battery Charger and SMBus Host. (See ALARM_MODE bit flag definition.)
    pub charger_mode: bool,

    /// CAPACITY_MODE bit indicates if capacity information will be reported in mA/mAh or 10mW/10mWh.
    /// When set, the capacity information will be reported in 10mW/10mWh as appropriate. When cleared, the
    /// capacity information will be reported in mA/mAh as appropriate.
    ///
    /// After changing the CAPACITY_MODE bit, all related values (such as AtRate()) should be re-written while
    /// the new mode is active. This is because changes made to the CAPACITY_MODE bit do not retroactively
    /// affect values which may have been previously written in another mode. For example, a value written to
    /// AtRate() while the CAPACITY_MODE bit was 0 will cause AtRate calculations to be made using the mAH
    /// value. Changing the CAPACITY_MODE bit to 1 will not automatically cause all the AtRate calculations to
    /// be re-calculated using the 10mWH equivalent, although this is permitted, it is not required.
    pub capacity_mode: bool,
}

/// Return value of the battery_status() function (0x16). See the SBS spec for more information.
#[bitfield(u16, defmt = cfg(feature = "defmt"))]
#[derive(Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BatteryStatusFields {
    /// Error codes from the Smart Battery. See ErrorCode enum fields for detailed documentation on what each
    /// error code entails.
    #[bits(4)]
    pub error_code: ErrorCode,

    /// FULLY_DISCHARGED bit is set when the Smart Battery determines that it has supplied all the charge it
    /// can. Discharge should be stopped soon.
    ///
    /// This bit will be cleared when the RelativeStateOfCharge() is
    /// greater than or equal to 20%. This status bit may be set prior to the
    /// ‘TERMINATE_DISCHARGE_ALARM’ as an early or first level warning of end of battery charge.
    pub fully_discharged: bool,

    /// FULLY_CHARGED bit is set when the Smart Battery determines that has reached a full charge point.
    ///
    ///  This bit will be cleared when the battery may want to be charged again, which is chemistry and
    /// manufacturer specific.
    pub fully_charged: bool,

    /// DISCHARGING bit is set when the Smart Battery determines that it is not being charged.
    /// This bit will be cleared when the battery detects that it is being charged.
    pub discharging: bool,

    /// INITIALIZED bit is SET when the Smart Battery electronics are calibrated or configured for the first time,
    /// typically at the time of battery pack assembly or manufacture.
    ///
    /// It will be cleared when the battery detects that this calibration or configuration data has been lost or
    /// altered and a significant degradation in accuracy is possible.
    ///
    /// The INITIALIZED status bit is the second and more serious signal from the Smart Battery that it has
    /// perhaps lost the ability to determine the present state-of-charge. As a result other data values required by
    /// this specification may be inaccurate.
    ///
    /// (The first signal from the Smart Battery is typically the CONDITION_FLAG found in the BatteryMode()
    /// register.)
    pub initialized: bool,

    /// REMAINING_TIME_ALARM bit is set when the Smart Battery detects that the estimated remaining
    /// time at the present discharge rate represented by the value in AverageTimeToEmpty() is less than that set by
    /// the RemainingTimeAlarm() function.
    ///
    /// This bit will be cleared when either the value set by the RemainingTimeAlarm() function is lower than the
    /// AverageTimeToEmpty() or when the AverageTimeToEmpty() is increased by charging the Smart Battery
    /// or decreasing the discharge rate.
    ///
    /// (NOTE: This Alarm bit can be disabled by writing zero to the RemainingTimeAlarm() value.)
    pub remaining_time_alarm: bool,

    /// REMAINING_CAPACITY_ALARM bit is set when the Smart Battery detects that its
    /// RemainingCapacity() is less than that set by the RemainingCapacityAlarm() function.
    ///
    /// This bit will be cleared when either the value set by the RemainingCapacityAlarm() function is lower than the
    /// RemainingCapacity() or when the RemainingCapacity() is increased by charging the Smart Battery.
    ///
    /// (NOTE: This Alarm bit can be disabled by writing zero to the RemainingCapacityAlarm() value.)
    pub remaining_capacity_alarm: bool,

    __: bool,

    /// TERMINATE_DISCHARGE_ALARM bit is set when the Smart Battery determines that it has supplied
    /// all the charge it can at the present discharge rate. Discharge should be stopped as soon as possible.
    ///
    /// This bit will be cleared when the Smart Battery detects that the discharge has stopped or that the rate
    /// has lessened.
    /// (Note that since this is rate dependent, it may occur at a high discharge rate and disappear when the
    /// discharge rate has slowed such that the Smart Battery can continue to be discharged at the lower rate.)
    pub terminate_discharge_alarm: bool,

    /// OVER_TEMP_ALARM bit will be set when the Smart Battery detects that its internal temperature is
    /// greater than a preset allowable limit. When this bit is set, charging should be stopped as soon as possible.
    /// The Smart Battery may not yet be in a Fully Charged state.
    /// Charging is effectively ‘suspended,’ usually temporarily. Charging may resume when the Smart Battery
    /// detects that its internal temperature is below a preset limit to allow charging again. (This limit may be a
    /// different value than what caused the original alarm.)
    ///
    /// This bit is cleared when the internal temperature has dropped below an acceptable limit, which may or may
    /// not be the original alarm threshold (although charging may not always resume at this point.)
    pub over_temp_alarm: bool,

    __: bool,

    /// TERMINATE_CHARGE_ALARM bit is set when charging should be stopped but the Smart Battery may
    /// not yet be in a Fully Charged state. Charging is effectively ‘suspended,’ usually temporarily. Charging may
    /// resume when the Smart Battery detects that its charging parameters are back in allowable ranges and
    /// ChargingVoltage() and ChargingCurrent() values are both returned to non-zero values.
    ///
    /// This bit is cleared when the Smart Battery detects that it is no longer being charged.
    pub terminate_charge_alarm: bool,

    /// OVER_CHARGED_ALARM bit is set whenever the Smart Battery detects that it is being charged
    /// beyond a Fully Charged state. When this bit is set, charging should be completely stopped as soon as
    /// possible. Charging further can result in permanent damage to the battery.
    ///
    /// This bit will be cleared when the Smart Battery detects that it is no longer being charged. Charging should
    /// not automatically restart.
    pub over_charged_alarm: bool,
}

/// Return value of the specification_info() function (0x1a). See the SBS spec for more information.
#[bitfield(u16, defmt = cfg(feature = "defmt"))]
pub struct SpecificationInfoFields {
    /// Revision of the SBS spec supported by this Smart Battery.
    /// See Revision enum fields for detailed documentation.
    #[bits(4)]
    pub revision: Revision,

    /// Version of the SBS spec supported by this Smart Battery.
    /// See Version enum fields for detailed documentation.
    #[bits(4)]
    pub version: Version,

    /// 0 - 3 (multiplies voltages* by 10 ^ VScale).
    #[bits(4)]
    pub v_scale: u8,

    /// 0 - 3 (multiplies currents* and capacities by 10 ^ IPScale).
    #[bits(4)]
    pub ip_scale: u8,
}

/// Blocking Smart Battery methods.
pub trait SmartBattery: ErrorType {
    /// 0x01
    ///
    /// Gets the Low Capacity alarm threshold value. Whenever the RemainingCapacity() falls below the
    /// Low Capacity value, the Smart Battery sends AlarmWarning() messages to the SMBus Host with the
    /// REMAINING_CAPACITY_ALARM bit set. A Low Capacity value of 0 disables this alarm.
    /// (If the ALARM_MODE bit is set in BatteryMode() then the AlarmWarning() message is disabled for a set
    /// period of time. See the BatteryMode() function for further information.)
    ///
    /// The Low Capacity value is set to 10% of design capacity at time of manufacture. The Low Capacity value
    /// will remain unchanged until altered by the RemainingCapacityAlarm() function. The Low Capacity value
    /// may be expressed in either capacity (mAh) or power (10mWh) depending on the setting of the
    /// BatteryMode()'s CAPACITY_MODE bit (see BatteryMode()).
    fn remaining_capacity_alarm(&mut self) -> Result<CapacityModeValue, Self::Error>;

    /// 0x01
    ///
    /// Sets the Low Capacity alarm threshold value. Whenever the RemainingCapacity() falls below the
    /// Low Capacity value, the Smart Battery sends AlarmWarning() messages to the SMBus Host with the
    /// REMAINING_CAPACITY_ALARM bit set. A Low Capacity value of 0 disables this alarm.
    /// (If the ALARM_MODE bit is set in BatteryMode() then the AlarmWarning() message is disabled for a set
    /// period of time. See the BatteryMode() function for further information.)
    ///
    /// The Low Capacity value is set to 10% of design capacity at time of manufacture. The Low Capacity value
    /// will remain unchanged until altered by the RemainingCapacityAlarm() function. The Low Capacity value
    /// may be expressed in either capacity (mAh) or power (10mWh) depending on the setting of the
    /// BatteryMode()'s CAPACITY_MODE bit (see BatteryMode()).
    fn set_remaining_capacity_alarm(&mut self, capacity: CapacityModeValue) -> Result<(), Self::Error>;

    /// 0x02
    ///
    /// Gets the Remaining Time alarm value. Whenever the AverageTimeToEmpty() falls below the
    /// Remaining Time value, the Smart Battery sends AlarmWarning() messages to the SMBus Host with the
    /// REMAINING_TIME_ALARM bit set. A Remaining Time value of 0 effectively disables this alarm.
    /// (If the ALARM_MODE bit is set in BatteryMode() then the AlarmWarning() message is disabled for a set
    /// period of time. See the BatteryMode() function for further information.)
    ///
    /// The Remaining Time value is set to 10 minutes at time of manufacture. The Remaining Time value will
    /// remain unchanged until altered by the RemainingTimeAlarm() function.
    fn remaining_time_alarm(&mut self) -> Result<Minutes, Self::Error>;

    /// 0x02
    ///
    /// Sets the Remaining Time alarm value. Whenever the AverageTimeToEmpty() falls below the
    /// Remaining Time value, the Smart Battery sends AlarmWarning() messages to the SMBus Host with the
    /// REMAINING_TIME_ALARM bit set. A Remaining Time value of 0 effectively disables this alarm.
    /// (If the ALARM_MODE bit is set in BatteryMode() then the AlarmWarning() message is disabled for a set
    /// period of time. See the BatteryMode() function for further information.)
    ///
    /// The Remaining Time value is set to 10 minutes at time of manufacture. The Remaining Time value will
    /// remain unchanged until altered by the RemainingTimeAlarm() function.
    fn set_remaining_time_alarm(&mut self, time: Minutes) -> Result<(), Self::Error>;

    /// 0x03
    ///
    /// This function reads the various battery operational modes and reports the battery’s capabilities, modes,
    /// and flags minor conditions requiring attention.
    ///
    /// See the SBS specification for detailed documentation.
    fn battery_mode(&mut self) -> Result<BatteryModeFields, Self::Error>;

    /// 0x03
    ///
    /// This function sets the various battery operational modes and reports the battery’s capabilities, modes,
    /// and flags minor conditions requiring attention. Note that not all fields are writeable.
    ///
    /// See the SBS specification for detailed documentation.
    fn set_battery_mode(&mut self, flags: BatteryModeFields) -> Result<(), Self::Error>;

    /// 0x04
    ///
    /// The AtRate() function is the first half of a two-function call-set used to set the AtRate value used in
    /// calculations made by the AtRateTimeToFull(), AtRateTimeToEmpty(), and AtRateOK() functions.
    ///
    /// The AtRate value may be expressed in either current (mA) or power (10mW) depending on the setting of the
    /// BatteryMode()'s CAPACITY_MODE bit. (Configuration of the CAPACITY_MODE bit will alter the
    /// calculation of AtRate functions. Changing the state of CAPACITY_MODE may require a re-write to the
    /// AtRate() function using the appropriate units.)
    fn at_rate(&mut self) -> Result<CapacityModeSignedValue, Self::Error>;

    /// 0x04
    ///
    /// The AtRate() function is the first half of a two-function call-set used to set the AtRate value used in
    /// calculations made by the AtRateTimeToFull(), AtRateTimeToEmpty(), and AtRateOK() functions.
    ///
    /// The AtRate value may be expressed in either current (mA) or power (10mW) depending on the setting of the
    /// BatteryMode()'s CAPACITY_MODE bit. (Configuration of the CAPACITY_MODE bit will alter the
    /// calculation of AtRate functions. Changing the state of CAPACITY_MODE may require a re-write to the
    /// AtRate() function using the appropriate units.)
    fn set_at_rate(&mut self, rate: CapacityModeSignedValue) -> Result<(), Self::Error>;

    /// 0x05
    ///
    /// Returns the predicted remaining time to fully charge the battery at the previously written AtRate value in mA.
    ///
    /// Note: This function is only required to return a value when the CAPACITY_MODE bit is cleared and the
    /// AtRate() value is written in mA units. If the CAPACITY_MODE bit is set, then AtRateTimeToFull() may
    /// return 65535 to indicate over-range and return an error code indicating overflow. Alternately, this function
    /// may return a remaining time to full based on a 10 mW value in AtRate().
    fn at_rate_time_to_full(&mut self) -> Result<Minutes, Self::Error>;

    /// 0x06
    ///
    /// Returns the predicted remaining operating time if the battery is discharged at the previously written AtRate
    /// value. (Result will depend on the setting of CAPACITY_MODE bit.)
    fn at_rate_time_to_empty(&mut self) -> Result<Minutes, Self::Error>;

    /// 0x07
    ///
    /// Returns a Boolean value that indicates whether or not the battery can deliver the previously written AtRate
    /// value of additional energy for 10 seconds (Boolean). If the AtRate value is zero or positive, the
    /// AtRateOK() function will ALWAYS return true. Result may depend on the setting of CAPACITY_MODE
    /// bit.
    fn at_rate_ok(&mut self) -> Result<bool, Self::Error>;

    /// 0x08
    ///
    /// Returns the cell-pack's internal temperature (°K). The actual operational temperature range will be defined
    /// at a pack level by a particular manufacturer.
    fn temperature(&mut self) -> Result<DeciKelvin, Self::Error>;

    /// 0x09
    ///
    /// Returns the cell-pack voltage (mV).
    fn voltage(&mut self) -> Result<MilliVolts, Self::Error>;

    /// 0x0A
    ///
    /// Returns the current being supplied (or accepted) through the battery's terminals (mA).
    fn current(&mut self) -> Result<MilliAmpsSigned, Self::Error>;

    /// 0x0B
    ///
    /// Returns a one-minute rolling average based on the current being supplied (or accepted) through the battery's
    /// terminals (mA). The AverageCurrent() function is expected to return meaningful values during the battery's
    /// first minute of operation.
    fn average_current(&mut self) -> Result<MilliAmpsSigned, Self::Error>;

    /// 0x0C
    ///
    /// Returns the expected margin of error (%) in the state of charge calculation. For example, when MaxError()
    /// returns 10% and RelativeStateOfCharge() returns 50%, the Relative StateOfCharge() is actually between 50
    /// and 60%. The MaxError() of a battery is expected to increase until the Smart Battery identifies a condition
    /// that will give it higher confidence in its own accuracy. For example, when a Smart Battery senses that it has
    /// been fully charged from a fully discharged state, it may use that information to reset or partially reset
    /// MaxError(). The Smart Battery can signal when MaxError() has become too high by setting the
    /// CONDITION_FLAG bit in BatteryMode().
    fn max_error(&mut self) -> Result<Percent, Self::Error>;

    /// 0x0D
    ///
    /// Returns the predicted remaining battery capacity expressed as a percentage of FullChargeCapacity() (%).
    fn relative_state_of_charge(&mut self) -> Result<Percent, Self::Error>;

    /// 0x0E
    ///
    /// Returns the predicted remaining battery capacity expressed as a percentage of DesignCapacity() (%).
    ///
    /// Note that AbsoluteStateOfCharge() can return values greater than 100%.
    fn absolute_state_of_charge(&mut self) -> Result<Percent, Self::Error>;

    /// 0x0F
    ///
    /// Returns the predicted remaining battery capacity. The RemainingCapacity() capacity value is expressed in
    /// either current (mAh at a C/5 discharge rate) or power (10mWh at a P/5 discharge rate) depending on the
    /// setting of the BatteryMode()'s CAPACITY_MODE bit.
    fn remaining_capacity(&mut self) -> Result<CapacityModeValue, Self::Error>;

    /// 0x10
    ///
    /// Returns the predicted pack capacity when it is fully charged. The FullChargeCapacity() value is expressed
    /// in either current (mAh at a C/5 discharge rate) or power (10mWh at a P/5 discharge rate) depending on the
    /// setting of the BatteryMode()'s CAPACITY_MODE bit.
    fn full_charge_capacity(&mut self) -> Result<CapacityModeValue, Self::Error>;

    /// 0x11
    ///
    /// Returns the predicted remaining battery life at the present rate of discharge (minutes). The
    /// RunTimeToEmpty() value is calculated based on either current or power depending on the setting of the
    /// BatteryMode()'s CAPACITY_MODE bit. This is an important distinction because use of the wrong
    /// calculation mode may result in inaccurate return values.
    ///
    /// 65,535 indicates battery is not being discharged.
    fn run_time_to_empty(&mut self) -> Result<Minutes, Self::Error>;

    /// 0x12
    ///
    /// Returns a one-minute rolling average of the predicted remaining battery life (minutes). The
    /// AverageTimeToEmpty() value is calculated based on either current or power depending on the setting of
    /// the BatteryMode()'s CAPACITY_MODE bit. This is an important distinction because use of the wrong
    /// calculation mode may result in inaccurate return values.
    ///
    /// 65,535 indicates battery is not being discharged.
    fn average_time_to_empty(&mut self) -> Result<Minutes, Self::Error>;

    /// 0x13
    ///
    /// Returns a one minute rolling average of the predicted remaining time until the Smart Battery reaches full
    /// charge (minutes).
    ///
    /// 65,535 indicates the battery is not being charged.
    fn average_time_to_full(&mut self) -> Result<Minutes, Self::Error>;

    /// 0x16
    ///
    /// Returns the Smart Battery's status word which contains Alarm and Status bit flags. Some of the
    /// BatteryStatus() flags (REMAINING_CAPACITY_ALARM and REMAINING_TIME_ALARM) are
    /// calculated based on either current or power depending on the setting of the BatteryMode()'s
    /// CAPACITY_MODE bit. This is important because use of the wrong calculation mode may result in an
    /// inaccurate alarm.
    fn battery_status(&mut self) -> Result<BatteryStatusFields, Self::Error>;

    /// 0x17
    ///
    /// Returns the number of cycles the battery has experienced. A cycle is defined as:
    ///
    /// An amount of discharge approximately equal to the value of DesignCapacity.
    fn cycle_count(&mut self) -> Result<Cycles, Self::Error>;

    /// 0x18
    ///
    /// Returns the theoretical capacity of a new pack. The DesignCapacity() value is expressed in either current
    /// (mAh at a C/5 discharge rate) or power (10mWh at a P/5 discharge rate) depending on the setting of the
    /// BatteryMode()'s CAPACITY_MODE bit.
    fn design_capacity(&mut self) -> Result<CapacityModeValue, Self::Error>;

    /// 0x19
    ///
    /// Returns the theoretical voltage of a new pack (mV).
    fn design_voltage(&mut self) -> Result<MilliVolts, Self::Error>;

    /// 0x1A
    ///
    /// Returns the version number of the Smart Battery specification the battery pack supports, as well as voltage
    /// and current and capacity scaling information in a packed unsigned integer. Power scaling is the product of
    /// the voltage scaling times the current scaling.
    /// These scaling functions do NOT affect ChargingCurrent() and ChargingVoltage() values.
    /// A Smart Battery Charger cannot be assumed to know this scaling information. (However, a ‘Level 3’
    /// or ‘Host Controlled’ Smart Battery Charger may read this value if required for specific
    /// applications.)
    fn specification_info(&mut self) -> Result<SpecificationInfoFields, Self::Error>;

    /// 0x1B
    ///
    /// This function returns the date the cell pack was manufactured.
    fn manufacture_date(&mut self) -> Result<ManufactureDate, Self::Error>;

    /// 0x1C
    ///
    /// This function is used to return a serial number. This number when combined with the ManufacturerName(),
    /// the DeviceName(), and the ManufactureDate() will uniquely identify the battery (unsigned int).
    fn serial_number(&mut self) -> Result<u16, Self::Error>;

    /// 0x20
    ///
    /// This function accepts a mutable buffer of u8s and returns it filled with a **null-terminated** character array
    /// containing the battery's manufacturer's name. For example, "MyBattCo\0" would identify the Smart Battery's
    /// manufacturer as MyBattCo.
    fn manufacturer_name(&mut self, name: &mut [u8]) -> Result<(), Self::Error>;

    /// 0x21
    ///
    /// This function accepts a mutable buffer of u8s and returns it filled with a **null-terminated** character array
    /// that contains the battery's name. For example, a DeviceName() of "MBC101\0" would indicate that
    /// the battery is a model MBC101.
    fn device_name(&mut self, name: &mut [u8]) -> Result<(), Self::Error>;

    /// 0x22
    ///
    /// This function accepts a mutable buffer of u8s and returns it filled with a **null-terminated** character array
    /// that contains the battery's chemistry. For example, if the DeviceChemistry() function returns "NiMH\0",
    /// the battery pack would contain nickel metal hydride cells.
    fn device_chemistry(&mut self, chemistry: &mut [u8]) -> Result<(), Self::Error>;
}

impl<T: SmartBattery + ?Sized> SmartBattery for &mut T {
    #[inline]
    fn remaining_capacity_alarm(&mut self) -> Result<CapacityModeValue, Self::Error> {
        T::remaining_capacity_alarm(self)
    }

    fn set_remaining_capacity_alarm(&mut self, capacity: CapacityModeValue) -> Result<(), Self::Error> {
        T::set_remaining_capacity_alarm(self, capacity)
    }

    #[inline]
    fn remaining_time_alarm(&mut self) -> Result<Minutes, Self::Error> {
        T::remaining_time_alarm(self)
    }

    fn set_remaining_time_alarm(&mut self, time: Minutes) -> Result<(), Self::Error> {
        T::set_remaining_time_alarm(self, time)
    }

    #[inline]
    fn battery_mode(&mut self) -> Result<BatteryModeFields, Self::Error> {
        T::battery_mode(self)
    }

    fn set_battery_mode(&mut self, flags: BatteryModeFields) -> Result<(), Self::Error> {
        T::set_battery_mode(self, flags)
    }

    #[inline]
    fn at_rate(&mut self) -> Result<CapacityModeSignedValue, Self::Error> {
        T::at_rate(self)
    }

    fn set_at_rate(&mut self, rate: CapacityModeSignedValue) -> Result<(), Self::Error> {
        T::set_at_rate(self, rate)
    }

    #[inline]
    fn at_rate_time_to_full(&mut self) -> Result<Minutes, Self::Error> {
        T::at_rate_time_to_full(self)
    }

    #[inline]
    fn at_rate_time_to_empty(&mut self) -> Result<Minutes, Self::Error> {
        T::at_rate_time_to_empty(self)
    }

    #[inline]
    fn at_rate_ok(&mut self) -> Result<bool, Self::Error> {
        T::at_rate_ok(self)
    }

    #[inline]
    fn temperature(&mut self) -> Result<DeciKelvin, Self::Error> {
        T::temperature(self)
    }

    #[inline]
    fn voltage(&mut self) -> Result<MilliVolts, Self::Error> {
        T::voltage(self)
    }

    #[inline]
    fn current(&mut self) -> Result<MilliAmpsSigned, Self::Error> {
        T::current(self)
    }

    #[inline]
    fn average_current(&mut self) -> Result<MilliAmpsSigned, Self::Error> {
        T::average_current(self)
    }

    #[inline]
    fn max_error(&mut self) -> Result<Percent, Self::Error> {
        T::max_error(self)
    }

    #[inline]
    fn relative_state_of_charge(&mut self) -> Result<Percent, Self::Error> {
        T::relative_state_of_charge(self)
    }

    #[inline]
    fn absolute_state_of_charge(&mut self) -> Result<Percent, Self::Error> {
        T::absolute_state_of_charge(self)
    }

    #[inline]
    fn remaining_capacity(&mut self) -> Result<CapacityModeValue, Self::Error> {
        T::remaining_capacity(self)
    }

    #[inline]
    fn full_charge_capacity(&mut self) -> Result<CapacityModeValue, Self::Error> {
        T::full_charge_capacity(self)
    }

    #[inline]
    fn run_time_to_empty(&mut self) -> Result<Minutes, Self::Error> {
        T::run_time_to_empty(self)
    }

    #[inline]
    fn average_time_to_empty(&mut self) -> Result<Minutes, Self::Error> {
        T::average_time_to_empty(self)
    }

    #[inline]
    fn average_time_to_full(&mut self) -> Result<Minutes, Self::Error> {
        T::average_time_to_full(self)
    }

    #[inline]
    fn battery_status(&mut self) -> Result<BatteryStatusFields, Self::Error> {
        T::battery_status(self)
    }
    #[inline]
    fn cycle_count(&mut self) -> Result<Cycles, Self::Error> {
        T::cycle_count(self)
    }

    #[inline]
    fn design_capacity(&mut self) -> Result<CapacityModeValue, Self::Error> {
        T::design_capacity(self)
    }

    #[inline]
    fn design_voltage(&mut self) -> Result<MilliVolts, Self::Error> {
        T::design_voltage(self)
    }

    #[inline]
    fn specification_info(&mut self) -> Result<SpecificationInfoFields, Self::Error> {
        T::specification_info(self)
    }

    #[inline]
    fn manufacture_date(&mut self) -> Result<ManufactureDate, Self::Error> {
        T::manufacture_date(self)
    }

    #[inline]
    fn serial_number(&mut self) -> Result<u16, Self::Error> {
        T::serial_number(self)
    }

    #[inline]
    fn manufacturer_name(&mut self, name: &mut [u8]) -> Result<(), Self::Error> {
        T::manufacturer_name(self, name)
    }

    #[inline]
    fn device_name(&mut self, name: &mut [u8]) -> Result<(), Self::Error> {
        T::device_name(self, name)
    }

    #[inline]
    fn device_chemistry(&mut self, chemistry: &mut [u8]) -> Result<(), Self::Error> {
        T::device_chemistry(self, chemistry)
    }
}
