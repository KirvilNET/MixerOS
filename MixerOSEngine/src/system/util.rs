use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BitDepth {
  BIT8,
  BIT16,
  BIT24,
  BIT32
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SampleRate {
  Hz44100 = 441000,
  Hz48000 = 48000,
  Hz96000 = 96000,
  Hz192000 = 192000
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum DASPStatus {
  STARTING,
  ONLINE,
  WORKING,
  OFFLINE,
  FAULT
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
