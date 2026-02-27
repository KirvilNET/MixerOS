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

  pub fn run() {
    
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