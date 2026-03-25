use std::collections::{ HashMap };
use std::sync::{ Arc };
use std::process::{ Command, ExitStatus };

use tokio::sync::RwLock;
use yansi::Paint;

use jack::{ Client, ClientOptions, ClientStatus };

use crate::cli::table::LiveTable;
use crate::system::util::{ ChannelType, BusType, DASPStatus, SampleRate, get_sample_rate };
use crate::router::{ channel, bus };
use crate::dasp::processor::*;

pub struct Engine {
  name: String,
  kernel_manager: Arc<KernelManager>,
  client: Option<Client>,
  status: Option<ClientStatus>,
  channels: HashMap<usize, Arc<RwLock<channel::ChannelStrip>>>,
  buses: HashMap<usize, Arc<RwLock<bus::Bus>>>,
  sample_rate: SampleRate,
  buffer_size: usize,
  table: LiveTable,
  dasp_status: DASPStatus,
}

#[derive(Debug)]
pub enum EngineError {
  MaxChannels,
  ChannelDoesNotExist,
  NoDevices,
  NoGPU,
  JackServerFailedToStart
}

impl Engine {
  pub fn new(name: String, ch: usize, buses: usize, sample_rate: SampleRate, buffer_size: usize, table: LiveTable) -> Self {
    let channel_strips: HashMap<usize, Arc<RwLock<channel::ChannelStrip>>> = HashMap::with_capacity(ch);
    let buses: HashMap<usize, Arc<RwLock<bus::Bus>>> = HashMap::with_capacity(buses);

    let kernel_manager = KernelManager::new(None).expect("Could not find any OpenCL3 compatible platforms/devices");

    Self {
      name,
      kernel_manager: Arc::new(kernel_manager),
      client: None,
      status: None,
      channels: channel_strips, 
      buses, 
      sample_rate,
      buffer_size,
      table,
      dasp_status: DASPStatus::STARTING,
    }
  }

  fn init(&mut self) -> Result<(), EngineError>{
    #[cfg(target_os = "macos")]
    let audio_backend: String = "coreaudio".to_string();

    #[cfg(target_os = "windows")]
    let audio_backend: String = "wasapi".to_string();

    #[cfg(target_os = "linux")]
    let audio_backend: String = "alsa".to_string();

    let mut client_options = ClientOptions::empty();

    client_options.insert(ClientOptions::USE_EXACT_NAME);
    client_options.insert(ClientOptions::NO_START_SERVER);

    let sample_rate = get_sample_rate(self.sample_rate);
    let is_running = Command::new("jack_control").arg("status").spawn().unwrap().wait().unwrap();

    if 1 == ExitStatus::code(&is_running).unwrap() {
      println!("Jack server already started");
    } else {
      let jackd_cmd = Command::new("jackd")
        .arg("-r")
        .arg(format!("-d{}", audio_backend))
        .arg(format!("-p {}", self.buffer_size))
        .arg(format!("-r {}", sample_rate))
        .arg("&")
        .spawn()
        .unwrap()
        .wait();

      if jackd_cmd.unwrap().success() {
        print!("Started the Jack Audio server")
      } else {
        return Err(EngineError::JackServerFailedToStart)
      }
    }


    if self.client.is_none() && self.status.is_none() {
      let (client, status) = Client::new(&self.name, client_options).unwrap();

      self.client = Some(client);
      self.status = Some(status);
    }

    let mut talkback = channel::ChannelStrip::new(
      "Talkback".to_string(),
      1, 
      ChannelType::SYSTEM, 
      false,
      self.sample_rate, 
      self.buffer_size,
      self.kernel_manager.clone()
    );

    let mut comms = channel::ChannelStrip::new(
      "Comms".to_string(),
      2, 
      ChannelType::SYSTEM,
      false,
      self.sample_rate, 
      self.buffer_size,
      self.kernel_manager.clone()
    );

    let mut mains = bus::Bus::new(
      "Mains".to_string(),
      1, 
      BusType::MAINS,
      2,
      self.sample_rate, 
      self.buffer_size,
      self.kernel_manager.clone()
    );

    let mut monitor = bus::Bus::new(
      "Monitor Left".to_string(), 
      2, 
      BusType::MONITOR,
      2,
      self.sample_rate, 
      self.buffer_size,
      self.kernel_manager.clone()
    );

    mains.add_modules();
    self.table.add_row(mains.table_row());

    monitor.add_modules();
    self.table.add_row(monitor.table_row());

    talkback.add_modules();
    self.table.add_row(talkback.table_row());

    comms.add_modules();
    self.table.add_row(comms.table_row());

    self.add_bus(1, mains).ok();
    self.add_bus(2, monitor).ok();

    self.add_channel(1, talkback).ok();
    self.add_channel(2, comms).ok();

    

    for ch in 3..self.channels.capacity() {
      let mut strip = channel::ChannelStrip::new(
        format!("Channel: {}", ch - 3), 
        ch, 
        ChannelType::USER,
        true, 
        self.sample_rate, 
        self.buffer_size,
        self.kernel_manager.clone()
      );

      strip.add_modules();
      self.add_channel(ch, strip).ok();
    }

    for bus in 3..self.buses.capacity() {
      let mut bus_strip = bus::Bus::new(
        format!("Bus: {}", bus - 3), 
        bus,
        BusType::AUX, 
        2,
        self.sample_rate,  
        self.buffer_size,
        self.kernel_manager.clone()
      );

      bus_strip.add_modules();
      self.add_bus(bus, bus_strip).ok();
    }

    let _ = self.client.as_mut().unwrap().transport().start();

    Ok(())
  }

  pub fn start(&mut self) -> Result<(), EngineError> {
    println!("Starting MixerOS Engine");

    self.init()
  }

  pub async fn run(&mut self) {
    
    for (id, channel) in self.channels.iter() {
        println!("Starting Channel {}", id);
        let mut ch = channel.write().await;

        match ch.run().await {
            Ok(_) =>  {
              println!("Successfully started channel {}", id).green();
            },
            Err(_) => {
              println!("Failed to start channel {}", id).red();
            },
        }
       
    }

    for (id, buses) in self.buses.iter() {
      println!("Starting Channel {}", id);
      let mut bus = buses.write().await;

      match bus.run().await {
          Ok(_) =>  {
            println!("Successfully started bus {}", id).green();
          },
          Err(_) => {
            println!("Failed to start bus {}", id).red();
          },
      }
    }

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