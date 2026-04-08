use std::collections::{ HashMap };
use std::fs;
use std::io::{BufRead, BufReader};
use std::sync::{ Arc };
use std::process::{ Command, Stdio };
use std::path::{ PathBuf };

use tokio::sync::RwLock;
//use tokio::process::Command;
use yansi::Paint;

use mixeros_jack::{ Client, ClientOptions, ClientStatus };

use crate::cli::table::LiveTable;
use crate::system::state::{BusConfig, ChannelConfig};
use crate::system::util::{ ChannelPermissions, BusType, DASPStatus, SampleRate, get_sample_rate };
use crate::router::{ channel, bus };
use crate::dasp::processor::*;

pub struct Engine {
  name: String,
  kernel_manager: Arc<KernelManager>,
  client: Option<Client>,
  status: Option<ClientStatus>,
  channels: HashMap<u32, Arc<RwLock<channel::ChannelStrip>>>,
  buses: HashMap<u32, Arc<RwLock<bus::Bus>>>,
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
  JackError(mixeros_jack::Error)
}

fn default_modules() -> Vec<PathBuf> {
  let mut vec: Vec<PathBuf> = Vec::new();
  let modules_kernel_path: PathBuf = match std::env::var_os("MODULES_KERNEL_PATH") {
    Some(path) => PathBuf::from(path),
    None => PathBuf::from("./shaders"),
  };
  
  for entry in fs::read_dir(modules_kernel_path).unwrap() {
    let entry = entry.unwrap();

    vec.push(entry.path().to_path_buf());
  }

  vec
}

impl Engine {
  pub fn new(name: String, sample_rate: SampleRate, buffer_size: usize, table: LiveTable, additional_modules: Option<Vec<PathBuf>>) -> Self {
    let channel_strips: HashMap<u32, Arc<RwLock<channel::ChannelStrip>>> = HashMap::new();
    let buses: HashMap<u32, Arc<RwLock<bus::Bus>>> = HashMap::new();
    let mut paths: Vec<PathBuf> = default_modules();

    match additional_modules {
        Some(mut x) => paths.append(&mut x),
        None => {},
    }

    let kernel_manager = Self::init_kernel_manager(paths);
 
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

  fn init_kernel_manager(paths: Vec<PathBuf>) -> KernelManager {
    let mut kernel_manager: KernelManager = KernelManager::new(None).expect("Could not find any OpenCL compatible platform/device");
    for path in paths {
      let _ = kernel_manager.add_program(path.as_os_str().to_str().unwrap(), &[path.file_name().unwrap().to_str().unwrap()]);
    }

    kernel_manager
  }

  async fn init(&mut self, ch: Vec<ChannelConfig>, buses: Vec<BusConfig>) -> Result<(), EngineError>{
    let mut jackd_started: bool = false;

    #[cfg(target_os = "macos")]
    let audio_backend: String = "coreaudio".to_string();

    #[cfg(target_os = "windows")]
    let audio_backend: String = "wasapi".to_string();

    #[cfg(target_os = "linux")]
    let audio_backend: String = "alsa".to_string();

    #[cfg(feature = "debug")]
    let audio_backend = "dummy".to_string();

    let mut client_options = ClientOptions::empty();
    let sample_rate = get_sample_rate(self.sample_rate);

    client_options.set(ClientOptions::USE_EXACT_NAME, true);
    client_options.set(ClientOptions::NO_START_SERVER, true);

    println!("Starting jack server");
    println!("Killing any old servers");
    let _ = Command::new("pkill").arg("jackd").spawn().unwrap().wait();
    let _ = Command::new("killall").arg("-9").arg("jackd").spawn().unwrap().wait();

    let mut jackd_cmd = Command::new("jackd")
      //.arg("-r")
      .arg(format!("-d{}", audio_backend))
      .arg(format!("-p{}", self.buffer_size))
      .arg(format!("-r{}", sample_rate))
      //.arg("--port-max 8192")
      .args([">", "./tmp/jack.log 2>&1"]) // TODO change this to log to a file
      .arg("&")
      .stdin(Stdio::null())
      .stdout(Stdio::piped())
      .spawn()
      .unwrap();

    while jackd_started == false {
      println!("Waiting for Jack Server to initilize");
      let jack_stdout = jackd_cmd.stdout.take().expect("Could not get stdout of jackd command");
      let reader = BufReader::new(jack_stdout);

      for line in reader.lines() {
        let line = line.expect("Couldn't read line");
        println!("{}", line);
        if line.contains("driver is running...") {
          jackd_started = true;
          println!("Started the Jack server");
          break;
        }
      }

      jackd_cmd.try_wait().inspect(|f| {
        match f {
            Some(x) => {
              println!("jackd exited with: {}", x)
            },
            None => {},
        }
      }).expect("jackd Error");

      std::thread::sleep(std::time::Duration::from_secs(1));
    }


    let c = Client::new(&self.name.as_str(), client_options);
    let (client, status) = match c {
      Ok((client, status)) => {
        println!("Connected, status: {:?}", status);
        (client, status)
      },
      Err(e) => return {
        Err(EngineError::JackError(e))
      },
    };

    println!("Client: {:?}", status);

    self.client = Some(client);
    self.status = Some(status);

    

    let mut talkback = channel::ChannelStrip::new(
      "Talkback".to_string(),
      1, 
      ChannelPermissions::SYSTEM, 
      false,
      self.sample_rate, 
      self.buffer_size,
      self.kernel_manager.clone()
    );

    let mut comms = channel::ChannelStrip::new(
      "Intercom".to_string(),
      2, 
      ChannelPermissions::SYSTEM,
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

    //mains.add_modules();
    //self.table.add_row(mains.table_row());

    //monitor.add_modules();
    //self.table.add_row(monitor.table_row());

    //talkback.add_modules();
    //self.table.add_row(talkback.table_row());

    //comms.add_modules();
    //self.table.add_row(comms.table_row());

    self.add_bus(1, mains).ok();
    self.add_bus(2, monitor).ok();

    self.add_channel(1, talkback).ok();
    self.add_channel(2, comms).ok();

    

    for ch_cfg in ch {
      let mut strip = channel::ChannelStrip::new(
        ch_cfg.name, 
        ch_cfg.id, 
        ChannelPermissions::USER,
        ch_cfg.is_redundant, 
        self.sample_rate, 
        self.buffer_size,
        self.kernel_manager.clone()
      );

      //strip.add_modules();
      self.add_channel(ch_cfg.id, strip).ok();
    }

    for bus in buses {
      let mut bus_strip = bus::Bus::new(
        bus.name, 
        bus.id,
        BusType::AUX, 
        1,
        self.sample_rate,  
        self.buffer_size,
        self.kernel_manager.clone()
      );

      //bus_strip.add_modules();
      self.add_bus(bus.id, bus_strip).ok();
    }

    Ok(())
  }

  pub async fn start(&mut self, ch: Vec<ChannelConfig>, buses: Vec<BusConfig>) -> Result<(), EngineError> {
    println!("Starting MixerOS Engine");

    let _init = self.init(ch, buses).await.unwrap();
    let _ = self.client.as_mut().unwrap().transport().start();

    Ok(())
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

  pub fn add_channel(&mut self, channel_number: u32, ch: channel::ChannelStrip) -> Result<(), EngineError> {
    if self.channels.len() as u32 == u32::MAX {
      return Err(EngineError::MaxChannels)
    }

    self.channels.insert(channel_number, Arc::new(RwLock::new(ch)));
    Ok(())
  }

  pub fn remove_channel(&mut self, channel_number: u32) -> Result<&Arc<RwLock<channel::ChannelStrip>>, EngineError> {
    let channel = self.channels.get(&channel_number);
    if channel_number > u32::MAX {
      return Err(EngineError::ChannelDoesNotExist)
    } 

    match channel {
        Some(x) => Ok(x),
        None => Err(EngineError::ChannelDoesNotExist),
    }
    
  }

  pub fn get_channel(&self, channel: u32) -> Result<Arc<RwLock<channel::ChannelStrip>>, EngineError> {
    if self.channels.len() == usize::MAX {
      return Err(EngineError::MaxChannels)
    }

    let ch = self.channels.get(&channel);

    match ch {
        Some(x) => Ok(x.clone()),
        None => Err(EngineError::ChannelDoesNotExist),
    }
  }

  pub fn add_bus(&mut self, bus_number: u32, bus: bus::Bus) -> Result<(), EngineError> {
    if self.buses.len() as u32 == u32::MAX {
      return Err(EngineError::MaxChannels)
    }

    let bus = Arc::new(RwLock::new(bus));

    self.buses.insert(bus_number, bus);
    Ok(())
  }

  pub fn remove_bus(&mut self, channel_number: u32) -> Result<&Arc<RwLock<bus::Bus>>, EngineError> {
    let channel = self.buses.get(&channel_number);
    if channel_number > u32::MAX {
      return Err(EngineError::ChannelDoesNotExist)
    } 

    match channel {
        Some(x) => Ok(x),
        None => Err(EngineError::ChannelDoesNotExist),
    }
    
  }

  pub fn get_bus(&self, channel: u32) -> Result<Arc<RwLock<channel::ChannelStrip>>, EngineError> {
    if self.channels.len() == usize::MAX {
      return Err(EngineError::MaxChannels)
    }

    let ch = self.channels.get(&channel);

    match ch {
        Some(x) => Ok(x.clone()),
        None => Err(EngineError::ChannelDoesNotExist),
    }
  }

  pub fn get_channels(&self) -> HashMap<u32, Arc<RwLock<channel::ChannelStrip>>> {
    self.channels.clone()
  }

  pub fn get_buses(&self) -> HashMap<u32, Arc<RwLock<bus::Bus>>> {
    self.buses.clone()
  }
}