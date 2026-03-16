use std::collections::{ HashMap };
use std::sync::{ Arc };
use std::str::FromStr;
use cpal::traits::HostTrait;
use cpal::{ Host, HostId, host_from_id};
use tokio::sync::RwLock;

use crate::dasp::processor::gpu::boilerplate::VKbase;
use crate::system::util::{ BitDepth, ChannelType, DASPStatus, SampleRate };
use crate::router::{ channel, bus };
use crate::dasp::processor::processor::*;

pub struct Engine {
  host: Host,
  channels: HashMap<usize, Arc<RwLock<channel::ChannelStrip>>>,
  buses: HashMap<usize, Arc<RwLock<bus::Bus>>>,
  bit_depth: BitDepth,
  sample_rate: SampleRate,
  buffer_size: usize,
  dasp_status: DASPStatus,
  processor_def: Processor
}

#[derive(Debug)]
pub enum EngineError {
  MaxChannels,
  ChannelDoesNotExist,
  NoDevices,
  NoGPU,
}

impl Engine {
  pub fn new(host: Host, ch: usize, buses: usize, bit_depth: BitDepth, sample_rate: SampleRate, buffer_size: usize) -> Self {
    let channel_strips: HashMap<usize, Arc<RwLock<channel::ChannelStrip>>> = HashMap::with_capacity(ch);
    let buses: HashMap<usize, Arc<RwLock<bus::Bus>>> = HashMap::with_capacity(buses);
    let processor_def: Processor;

    match Self::dertermine_processor() {
        ProcessorType::CPU => {
          processor_def = Processor { processor_type: ProcessorType::CPU, processor: ProcessorUnit::CPU(Processor::create_cpu_processor(ch as u32, buffer_size as u32))}
        },
        ProcessorType::GPU => {
          processor_def = Processor { processor_type: ProcessorType::GPU, processor: ProcessorUnit::GPU(Processor::create_gpu_processor(ch as u32, buffer_size as u32))}
        },
    }

    Self {
      host,
      channels: channel_strips, 
      buses, 
      bit_depth,
      sample_rate,
      buffer_size, 
      dasp_status: DASPStatus::STARTING,
      processor_def
    }
  }

  fn init(&mut self) -> Result<(), EngineError>{

    let talkback = channel::ChannelStrip::new(
      "Talkback".to_string(), 
      1, 
      ChannelType::USER, 
      self.sample_rate, 
      self.host.default_input_device().ok_or(EngineError::NoDevices)?, 
      self.buffer_size as u32,
      self.processor_def.clone()
    );

    let comms = channel::ChannelStrip::new(
      "Comms".to_string(), 
      2, 
      ChannelType::USER, 
      self.sample_rate, 
      self.host.default_input_device().ok_or(EngineError::NoDevices)?, 
      self.buffer_size as u32,
      self.processor_def.clone()
    );

    let mains_l = bus::Bus::new(
      "Mains Left".to_string(), 
      1, 
      ChannelType::USER, 
      self.sample_rate, 
      self.host.default_output_device().ok_or(EngineError::NoDevices)?, 
      self.buffer_size as u32,
      self.processor_def.clone()
    );

    let mains_r = bus::Bus::new(
      "Mains Right".to_string(), 
      1, 
      ChannelType::USER, 
      self.sample_rate, 
      self.host.default_output_device().ok_or(EngineError::NoDevices)?, 
      self.buffer_size as u32,
      self.processor_def.clone()
    );

    let monitor_l = bus::Bus::new(
      "Monitor Left".to_string(), 
      2, 
      ChannelType::USER, 
      self.sample_rate, 
      self.host.default_output_device().ok_or(EngineError::NoDevices)?, 
      self.buffer_size as u32,
      self.processor_def.clone()
    );

    let monitor_r = bus::Bus::new(
      "Monitor Right".to_string(), 
      2, 
      ChannelType::USER, 
      self.sample_rate, 
      self.host.default_output_device().ok_or(EngineError::NoDevices)?, 
      self.buffer_size as u32,
      self.processor_def.clone()
    );

    self.add_bus(1, mains_l).ok();
    self.add_bus(2, monitor_l).ok();

    self.add_bus(1, mains_r).ok();
    self.add_bus(2, monitor_r).ok();

    self.add_channel(1, talkback).ok();
    self.add_channel(2, comms).ok();

    for ch in 2..self.channels.capacity() {
      let mut strip = channel::ChannelStrip::new(
         format!("Channel: {}", ch), 
         ch, 
         ChannelType::USER, 
         self.sample_rate, 
         self.host.default_input_device().ok_or(EngineError::NoDevices)?, 
         self.buffer_size as u32,
         self.processor_def.clone()
      );

      let _ = strip.create_input();

      self.add_channel(ch, strip).ok();
    }

    for bus in 3..self.channels.capacity() {
      let bus_strip = bus::Bus::new(
        format!("Channel: {}", bus), 
        bus, 
        ChannelType::USER, 
        self.sample_rate, 
        self.host.default_input_device().ok_or(EngineError::NoDevices)?,  
        self.buffer_size as u32,
        self.processor_def.clone()
      );
      

      self.add_bus(bus, bus_strip).ok();
    }

    Ok(())
  }

  fn dertermine_processor() -> ProcessorType {
    let processor_type: ProcessorType;

    match VKbase::check() {
        Ok(_) => {
          processor_type = ProcessorType::GPU;
        },
        Err(_) => {
          processor_type = ProcessorType::CPU;
        },
    }

    processor_type
  }

  pub fn start(&mut self) -> Result<(), EngineError> {
    Engine::init(self)
  }

  pub async fn run(&mut self) {
    
    self.dasp_status = DASPStatus::ONLINE;
  }

  pub fn add_channel(&mut self, channel_number: usize, ch: channel::ChannelStrip) -> Result<(), EngineError> {
    if self.channels.len() == usize::MAX {
      return Err(EngineError::MaxChannels)
    }

    self.channels.insert(channel_number, Arc::new(RwLock::new(ch)));
    Ok(())
  }

  pub fn remove_channel(&mut self, channel_number: usize) -> Result<&Arc<RwLock<channel::ChannelStrip>>, EngineError> {
    let channel = self.channels.get(&channel_number);
    if channel_number > usize::MAX {
      return Err(EngineError::ChannelDoesNotExist)
    } 

    match channel {
        Some(x) => Ok(x),
        None => Err(EngineError::ChannelDoesNotExist),
    }
    
  }

  pub fn get_channel(&mut self, channel: usize) -> Result<&Arc<RwLock<channel::ChannelStrip>>, EngineError> {
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

    let bus = Arc::new(RwLock::new(bus));

    self.buses.insert(bus_number, bus);
    Ok(())
  }

  pub fn remove_bus(&mut self, channel_number: usize) -> Result<&Arc<RwLock<channel::ChannelStrip>>, EngineError> {
    let channel = self.channels.get(&channel_number);
    if channel_number > usize::MAX {
      return Err(EngineError::ChannelDoesNotExist)
    } 

    match channel {
        Some(x) => Ok(x),
        None => Err(EngineError::ChannelDoesNotExist),
    }
    
  }

  pub fn get_bus(&mut self, channel: usize) -> Result<&Arc<RwLock<channel::ChannelStrip>>, EngineError> {
    if self.channels.len() == usize::MAX {
      return Err(EngineError::MaxChannels)
    }

    let ch = self.channels.get(&channel);

    match ch {
        Some(x) => Ok(x),
        None => Err(EngineError::ChannelDoesNotExist),
    }
  }

  pub fn get_channels(&mut self) -> HashMap<usize, Arc<RwLock<channel::ChannelStrip>>> {
    self.channels.clone()
  }

  pub fn get_buses(&mut self) -> HashMap<usize, Arc<RwLock<bus::Bus>>> {
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