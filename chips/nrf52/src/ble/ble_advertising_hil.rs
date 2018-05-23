//! Bluetooth Low Energy HIL
//!
//! ```text
//! Application
//!
//!           +------------------------------------------------+
//!           | Applications                                   |
//!           +------------------------------------------------+
//!
//! ```
//!
//! ```text
//! Host
//!
//!           +------------------------------------------------+
//!           | Generic Access Profile                         |
//!           +------------------------------------------------+
//!
//!           +------------------------------------------------+
//!           | Generic Attribute Profile                      |
//!           +------------------------------------------------+
//!
//!           +--------------------+      +-------------------+
//!           | Attribute Protocol |      | Security Manager  |
//!           +--------------------+      +-------------------+
//!
//!           +-----------------------------------------------+
//!           | Logical Link and Adaptation Protocol          |
//!           +-----------------------------------------------+
//!
//! ```
//!
//! ```text
//! Controller
//!
//!           +--------------------------------------------+
//!           | Host Controller Interface                  |
//!           +--------------------------------------------+
//!
//!           +------------------+      +------------------+
//!           | Link Layer       |      | Direct Test Mode |
//!           +------------------+      +------------------+
//!
//!           +--------------------------------------------+
//!           | Physical Layer                             |
//!           +--------------------------------------------+
//!
//! ```

use ble::ble_connection_driver::ConnectionData;
use kernel::ReturnCode;
use nrf5x::constants::BLE_T_IFS;
use core;

pub trait BleAdvertisementDriver {
    fn transmit_advertisement(&self);
    fn set_advertisement_data(&self, buf: &'static mut [u8], len: usize) -> &'static mut [u8];
    fn receive_advertisement(&self);

    fn set_receive_client(&self, client: &'static RxClient);
    fn set_transmit_client(&self, client: &'static TxClient);
    fn set_advertisement_client(&self, client: &'static AdvertisementClient);
}

pub trait BleConfig {
    fn set_tx_power(&self, power: u8) -> ReturnCode;
    fn set_channel(&self, channel: RadioChannel, address: u32, crcinit: u32);
    fn set_access_address(&self, aa: u32);
}

#[derive(Debug, Copy, Clone)]
pub enum DelayStartPoint {
    PacketEndBLEStandardDelay,
    PacketStartUsecDelay(u32),
    PacketEndUsecDelay(u32),
    AbsoluteTimestamp(u32),
    PreviousPacketStartUsecDelay(u32),
}

impl DelayStartPoint {
    pub fn value(self) -> u32 {
        match self {
            DelayStartPoint::PacketEndUsecDelay(v)
            | DelayStartPoint::PacketStartUsecDelay(v)
            | DelayStartPoint::PreviousPacketStartUsecDelay(v)
            | DelayStartPoint::AbsoluteTimestamp(v) => v,
            DelayStartPoint::PacketEndBLEStandardDelay => BLE_T_IFS,
        }
    }
}

pub enum PhyTransition {
    None,
    MoveToTX(DelayStartPoint),
    MoveToRX(DelayStartPoint, u32), //(schedule_rx_after_time, timeout)
}

pub enum ResponseAction {
    ScanResponse,
    Connection(ConnectionData),
}

pub enum ActionAfterTimerExpire {
    ContinueAdvertising,
    ContinueConnection(u32, u32),
}

pub enum ReadAction {
    SkipFrame,
    ReadFrame,
}

pub enum TxImmediate {
    GoToSleep,
    RespondAfterTifs,
    TX,
}

pub trait RxClient {
    fn receive_start(&self, buf: &'static mut [u8], len: u8) -> ReadAction;
    fn receive_end(
        &self,
        buf: &'static mut [u8],
        len: u8,
        result: ReturnCode,
        rx_timestamp: u32,
    ) -> PhyTransition;
}

pub trait TxClient {
    fn transmit_end(&self, result: ReturnCode) -> PhyTransition;
}

pub trait AdvertisementClient {
    fn advertisement_done(&self) -> TxImmediate;
    fn timer_expired(&self) -> PhyTransition;
}

// Bluetooth Core Specification:Vol. 6. Part B, section 1.4.1 Advertising and Data Channel Indices
#[derive(PartialEq, Debug, Copy, Clone)]
pub enum RadioChannel {
    DataChannel0 = 4,
    DataChannel1 = 6,
    DataChannel2 = 8,
    DataChannel3 = 10,
    DataChannel4 = 12,
    DataChannel5 = 14,
    DataChannel6 = 16,
    DataChannel7 = 18,
    DataChannel8 = 20,
    DataChannel9 = 22,
    DataChannel10 = 24,
    DataChannel11 = 28,
    DataChannel12 = 30,
    DataChannel13 = 32,
    DataChannel14 = 34,
    DataChannel15 = 36,
    DataChannel16 = 38,
    DataChannel17 = 40,
    DataChannel18 = 42,
    DataChannel19 = 44,
    DataChannel20 = 46,
    DataChannel21 = 48,
    DataChannel22 = 50,
    DataChannel23 = 52,
    DataChannel24 = 54,
    DataChannel25 = 56,
    DataChannel26 = 58,
    DataChannel27 = 60,
    DataChannel28 = 62,
    DataChannel29 = 64,
    DataChannel30 = 66,
    DataChannel31 = 68,
    DataChannel32 = 70,
    DataChannel33 = 72,
    DataChannel34 = 74,
    DataChannel35 = 76,
    DataChannel36 = 78,
    AdvertisingChannel37 = 2,
    AdvertisingChannel38 = 26,
    AdvertisingChannel39 = 80,
}

impl RadioChannel {
    pub fn get_next_advertising_channel(&self) -> Option<RadioChannel> {
        match *self {
            RadioChannel::AdvertisingChannel37 => Some(RadioChannel::AdvertisingChannel38),
            RadioChannel::AdvertisingChannel38 => Some(RadioChannel::AdvertisingChannel39),
            _ => None,
        }
    }
    pub fn get_channel_index(&self) -> u32 {
        match *self {
            RadioChannel::DataChannel0 => 0,
            RadioChannel::DataChannel1 => 1,
            RadioChannel::DataChannel2 => 2,
            RadioChannel::DataChannel3 => 3,
            RadioChannel::DataChannel4 => 4,
            RadioChannel::DataChannel5 => 5,
            RadioChannel::DataChannel6 => 6,
            RadioChannel::DataChannel7 => 7,
            RadioChannel::DataChannel8 => 8,
            RadioChannel::DataChannel9 => 9,
            RadioChannel::DataChannel10 => 10,
            RadioChannel::DataChannel11 => 11,
            RadioChannel::DataChannel12 => 12,
            RadioChannel::DataChannel13 => 13,
            RadioChannel::DataChannel14 => 14,
            RadioChannel::DataChannel15 => 15,
            RadioChannel::DataChannel16 => 16,
            RadioChannel::DataChannel17 => 17,
            RadioChannel::DataChannel18 => 18,
            RadioChannel::DataChannel19 => 19,
            RadioChannel::DataChannel20 => 20,
            RadioChannel::DataChannel21 => 21,
            RadioChannel::DataChannel22 => 22,
            RadioChannel::DataChannel23 => 23,
            RadioChannel::DataChannel24 => 24,
            RadioChannel::DataChannel25 => 25,
            RadioChannel::DataChannel26 => 26,
            RadioChannel::DataChannel27 => 27,
            RadioChannel::DataChannel28 => 28,
            RadioChannel::DataChannel29 => 29,
            RadioChannel::DataChannel30 => 30,
            RadioChannel::DataChannel31 => 31,
            RadioChannel::DataChannel32 => 32,
            RadioChannel::DataChannel33 => 33,
            RadioChannel::DataChannel34 => 34,
            RadioChannel::DataChannel35 => 35,
            RadioChannel::DataChannel36 => 36,
            RadioChannel::AdvertisingChannel37 => 37,
            RadioChannel::AdvertisingChannel38 => 38,
            RadioChannel::AdvertisingChannel39 => 39,
        }
    }
}

#[derive(Debug)]
pub struct NoDataChannelForIndex;

impl core::convert::TryFrom<u8> for RadioChannel {
    type Error = NoDataChannelForIndex;

    fn try_from(value: u8) -> Result<Self, NoDataChannelForIndex> {
        match value {
            0 => Ok(RadioChannel::DataChannel0),
            1 => Ok(RadioChannel::DataChannel1),
            2 => Ok(RadioChannel::DataChannel2),
            3 => Ok(RadioChannel::DataChannel3),
            4 => Ok(RadioChannel::DataChannel4),
            5 => Ok(RadioChannel::DataChannel5),
            6 => Ok(RadioChannel::DataChannel6),
            7 => Ok(RadioChannel::DataChannel7),
            8 => Ok(RadioChannel::DataChannel8),
            9 => Ok(RadioChannel::DataChannel9),
            10 => Ok(RadioChannel::DataChannel10),
            11 => Ok(RadioChannel::DataChannel11),
            12 => Ok(RadioChannel::DataChannel12),
            13 => Ok(RadioChannel::DataChannel13),
            14 => Ok(RadioChannel::DataChannel14),
            15 => Ok(RadioChannel::DataChannel15),
            16 => Ok(RadioChannel::DataChannel16),
            17 => Ok(RadioChannel::DataChannel17),
            18 => Ok(RadioChannel::DataChannel18),
            19 => Ok(RadioChannel::DataChannel19),
            20 => Ok(RadioChannel::DataChannel20),
            21 => Ok(RadioChannel::DataChannel21),
            22 => Ok(RadioChannel::DataChannel22),
            23 => Ok(RadioChannel::DataChannel23),
            24 => Ok(RadioChannel::DataChannel24),
            25 => Ok(RadioChannel::DataChannel25),
            26 => Ok(RadioChannel::DataChannel26),
            27 => Ok(RadioChannel::DataChannel27),
            28 => Ok(RadioChannel::DataChannel28),
            29 => Ok(RadioChannel::DataChannel29),
            30 => Ok(RadioChannel::DataChannel30),
            31 => Ok(RadioChannel::DataChannel31),
            32 => Ok(RadioChannel::DataChannel32),
            33 => Ok(RadioChannel::DataChannel33),
            34 => Ok(RadioChannel::DataChannel34),
            35 => Ok(RadioChannel::DataChannel35),
            36 => Ok(RadioChannel::DataChannel36),
            37 => Ok(RadioChannel::AdvertisingChannel37),
            38 => Ok(RadioChannel::AdvertisingChannel38),
            39 => Ok(RadioChannel::AdvertisingChannel39),
            _ => Err(NoDataChannelForIndex),
        }
    }
}
