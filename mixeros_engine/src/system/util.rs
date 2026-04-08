use serde::{Deserialize, Serialize};
use std::fmt::Debug;

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

pub enum AudioBackend {
  CoreAudio,
  Alsa,
  Wasapi,
  Asio,
}

pub fn get_sample_rate(rate: SampleRate) -> u32 {
  match rate {
    SampleRate::Hz44100 => 44100,
    SampleRate::Hz48000 => 48000,
    SampleRate::Hz96000 => 96000,
    SampleRate::Hz192000 => 192000,
  }
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DASPStatus {
  STARTING,
  ONLINE,
  WORKING,
  OFFLINE,
  FAULT
}

impl Debug for DASPStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::STARTING => write!(f, "STARTING"),
            Self::ONLINE => write!(f, "ONLINE"),
            Self::WORKING => write!(f, "WORKING"),
            Self::OFFLINE => write!(f, "OFFLINE"),
            Self::FAULT => write!(f, "FAULT"),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ChannelPermissions {
  USER,
  SYSTEM,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Deserialize)]
pub enum BusType {
  MAINS,
  MONITOR,
  GROUP,
  AUX,
  MATRIX
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum DASPProcessorType {
  CPU,
  GPU,
  NPU,
  DSP,
  FPGA,
  TPU,
  NONE
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EngineRole {
  Controller,
  Node,
  RedundancyController,
  RedundancyNode,
}