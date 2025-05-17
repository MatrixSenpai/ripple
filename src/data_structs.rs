use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::watch::Sender;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct SensorData {
    pub temperature: f32,
    pub pressure: Option<f32>,
    pub humidity: Option<f32>,
}

pub type SensorSender = Sender<'static, CriticalSectionRawMutex, SensorData, 5>;
