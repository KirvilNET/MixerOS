use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum BitDepth {
  BIT8,
  BIT16,
  BIT24,
  BIT32
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum SampleRate {
  Hz44100 = 441000,
  Hz48000 = 48000,
  Hz96000 = 96000,
  Hz192000 = 192000
}

pub fn get_sample_rate(rate: SampleRate) -> u32 {
  match rate {
    SampleRate::Hz44100 => 44100,
    SampleRate::Hz48000 => 48000,
    SampleRate::Hz96000 => 96000,
    SampleRate::Hz192000 => 192000,
  }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DASPStatus {
  STARTING,
  ONLINE,
  WORKING,
  OFFLINE,
  FAULT
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ChannelType {
  USER,
  SYSTEM,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BusType {
  GROUP,
  AUX,
  SUM
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DASPProcessorType {
  CPU,
  GPU,
  NONE
}
