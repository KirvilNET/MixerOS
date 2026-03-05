use std::collections::{ HashMap };
use std::sync::{ Arc, Mutex };
use std::str::FromStr;
use cpal::{Host, HostId, host_from_id};

use crate::system::util::{ BitDepth, DASPStatus, SampleRate };
use crate::router::{ channel, bus };

pub struct Engine {
  host: Host,
  channels: HashMap<usize, Arc<channel::ChannelStrip>>,
  buses: HashMap<usize, Arc<bus::Bus>>,
  bit_depth: BitDepth,
  sample_rate: SampleRate,
  dasp_status: DASPStatus
}

pub enum EngineError {
  MaxChannels,
  ChannelDoesNotExist
}

impl Engine {
  pub fn new(host: Host, ch: usize, buses: usize, bit_depth: BitDepth, sample_rate: SampleRate) -> Self {
    let channel_strips: HashMap<usize, Arc<channel::ChannelStrip>> = HashMap::with_capacity(ch);
    let buses: HashMap<usize, Arc<bus::Bus>> = HashMap::with_capacity(buses);

    Self {
      host, 
      channels: channel_strips, 
      buses, 
      bit_depth,
      sample_rate, 
      dasp_status: DASPStatus::STARTING 
    }
  }

  pub fn add_channel(&mut self, channel_number: usize, ch: channel::ChannelStrip) -> Result<(), EngineError> {
    if self.channels.len() == usize::MAX {
      return Err(EngineError::MaxChannels)
    }

    self.channels.insert(channel_number, ch.into()).expect("Could not add Channel");
    Ok(())
  }

  pub fn remove_channel(&mut self, channel_number: usize) -> Result<&Arc<channel::ChannelStrip>, EngineError> {
    let channel = self.channels.get(&channel_number);
    if channel_number > usize::MAX {
      return Err(EngineError::ChannelDoesNotExist)
    } 

    match channel {
        Some(x) => Ok(x),
        None => Err(EngineError::ChannelDoesNotExist),
    }
    
  }

  pub fn get_channel(&mut self, channel: usize) -> Result<&Arc<channel::ChannelStrip>, EngineError> {
    if self.channels.len() == usize::MAX {
      return Err(EngineError::MaxChannels)
    }

    let ch = self.channels.get(&channel);

    match ch {
        Some(x) => Ok(x),
        None => Err(EngineError::ChannelDoesNotExist),
    }
  }

  pub fn get_channels(&mut self) -> HashMap<usize, Arc<channel::ChannelStrip>> {
    self.channels.clone()
  }

  pub fn get_buses(&mut self) -> HashMap<usize, Arc<bus::Bus>> {
    self.buses.clone()
  }
}

pub fn select_host() -> Result<Host, cpal::HostUnavailable> {
  if cfg!(target_os = "macos") {
    return host_from_id(HostId::from_str("coreaudio")?)
  } else if cfg!(target_os = "windows") {
    return host_from_id(HostId::from_str("wasapi")?)
  } else if cfg!(target_os = "linux") {
    return host_from_id(HostId::from_str("alsa")?)
  } else {
    return Err(cpal::HostUnavailable)
  }
}