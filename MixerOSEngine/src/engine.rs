use std::collections::{ HashMap };
use std::fmt::format;
use std::sync::{ Arc, Mutex };
use std::str::FromStr;
use cpal::traits::HostTrait;
use cpal::{Host, HostId, StreamConfig, host_from_id};
use yansi::Paint;

use crate::system::util::{ BitDepth, ChannelType, DASPStatus, SampleRate, get_sample_rate };
use crate::router::{ channel, bus };

pub struct Engine {
  host: Host,
  channels: HashMap<usize, Arc<channel::ChannelStrip>>,
  buses: HashMap<usize, Arc<bus::Bus>>,
  bit_depth: BitDepth,
  sample_rate: SampleRate,
  buffer_size: usize,
  dasp_status: DASPStatus
}

pub enum EngineError {
  MaxChannels,
  ChannelDoesNotExist,
  NoDevices
}

impl Engine {
  pub fn new(host: Host, ch: usize, buses: usize, bit_depth: BitDepth, sample_rate: SampleRate, buffer_size: usize) -> Self {
    let channel_strips: HashMap<usize, Arc<channel::ChannelStrip>> = HashMap::with_capacity(ch + 2);
    let buses: HashMap<usize, Arc<bus::Bus>> = HashMap::with_capacity(buses + 3);

    Self {
      host, 
      channels: channel_strips, 
      buses, 
      bit_depth,
      sample_rate,
      buffer_size, 
      dasp_status: DASPStatus::STARTING 
    }
  }

  pub fn start(&mut self) -> Result<(), EngineError>{
    let talkback = channel::ChannelStrip::new("Talkback".to_string(), 
      1, 
      ChannelType::USER, 
      self.sample_rate, 
      self.host.default_input_device().ok_or(EngineError::NoDevices)?, 
      StreamConfig { channels: 1, sample_rate: get_sample_rate(self.sample_rate) as u32, buffer_size: cpal::BufferSize::Fixed(self.buffer_size as u32) }
    );

    let comms = channel::ChannelStrip::new("Talkback".to_string(), 
      2, 
      ChannelType::USER, 
      self.sample_rate, 
      self.host.default_input_device().ok_or(EngineError::NoDevices)?, 
      StreamConfig { channels: 2, sample_rate: get_sample_rate(self.sample_rate) as u32, buffer_size: cpal::BufferSize::Fixed(self.buffer_size as u32) }
    );

    let mains = bus::Bus::new("Talkback".to_string(), 
      1, 
      ChannelType::USER, 
      self.sample_rate, 
      self.host.default_input_device().ok_or(EngineError::NoDevices)?, 
      StreamConfig { channels: 2, sample_rate: get_sample_rate(self.sample_rate) as u32, buffer_size: cpal::BufferSize::Fixed(self.buffer_size as u32) }
    );

    let headphones = bus::Bus::new("Talkback".to_string(), 
      2, 
      ChannelType::USER, 
      self.sample_rate, 
      self.host.default_input_device().ok_or(EngineError::NoDevices)?, 
      StreamConfig { channels: 2, sample_rate: get_sample_rate(self.sample_rate) as u32, buffer_size: cpal::BufferSize::Fixed(self.buffer_size as u32) }
    );

    self.add_bus(1, mains).ok();
    self.add_bus(2, headphones).ok();

    self.add_channel(1, talkback).ok();
    self.add_channel(2, comms).ok();

    for ch in 2..self.channels.capacity() {
      let mut strip = channel::ChannelStrip::new(
         format!("Channel: {}", ch), 
         ch, 
         ChannelType::USER, 
         self.sample_rate, 
         self.host.default_input_device().ok_or(EngineError::NoDevices)?, 
         StreamConfig { 
          channels: 1, 
          sample_rate: get_sample_rate(self.sample_rate) as u32, 
          buffer_size: cpal::BufferSize::Fixed(self.buffer_size as u32) 
        }
      );

      let _ = strip.create_input();

      self.add_channel(ch, strip).ok();
    }

    for bus in 3..self.channels.capacity() {
      let mut bus_strip = bus::Bus::new(
        format!("Channel: {}", bus), 
        bus, 
        ChannelType::USER, 
        self.sample_rate, 
        self.host.default_input_device().ok_or(EngineError::NoDevices)?,  
        StreamConfig { 
          channels: 1, 
          sample_rate: get_sample_rate(self.sample_rate) as u32, 
          buffer_size: cpal::BufferSize::Fixed(self.buffer_size as u32) 
        }
      );

      self.add_bus(bus, bus_strip).ok();
    }

    Ok(())
  }

  pub async fn run(&mut self) {
    self.channels.iter();
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

  pub fn add_bus(&mut self, bus_number: usize, bus: bus::Bus) -> Result<(), EngineError> {
    if self.buses.len() == usize::MAX {
      return Err(EngineError::MaxChannels)
    }

    self.buses.insert(bus_number, bus.into()).expect("Could not add Channel");
    Ok(())
  }

  pub fn remove_bus(&mut self, channel_number: usize) -> Result<&Arc<channel::ChannelStrip>, EngineError> {
    let channel = self.channels.get(&channel_number);
    if channel_number > usize::MAX {
      return Err(EngineError::ChannelDoesNotExist)
    } 

    match channel {
        Some(x) => Ok(x),
        None => Err(EngineError::ChannelDoesNotExist),
    }
    
  }

  pub fn get_bus(&mut self, channel: usize) -> Result<&Arc<channel::ChannelStrip>, EngineError> {
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