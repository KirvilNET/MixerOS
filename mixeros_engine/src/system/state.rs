use mixeros_jack::Unowned;
use mixeros_protocol::mixeros_protocol_sys::proto::include::util_capnp::Permissions;
use serde::{ Deserialize, Serialize };
use tokio::{ fs };

use std::{ path::{Path, PathBuf}, sync::* };
use anyhow::{ anyhow };

use super::util::*;

#[derive(Debug)]
pub struct StateManager {
  dasp_state: Arc<Mutex<DASPState>>,
  config: Arc<Mutex<EngineConfig>>
}

#[derive(Debug)]
pub enum StateManagerError {
  FsReadError,
  FsWriteError,
  MutexLockError,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ChannelConfig {
  pub id: u32,
  pub name: String,
  pub permission: ChannelPermissions,
  pub is_redundant: bool,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct BusConfig {
  pub id: u32,
  pub name: String,
  pub permission: ChannelPermissions,
  pub is_redundant: bool,
  pub bus_type: BusType,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct EngineConfig {
  pub name: String,
  pub channels: Vec<ChannelConfig>,
  pub role: EngineRole,
  pub buses: Vec<BusConfig>,
  pub bit_depth: BitDepth,
  pub sample_rate: SampleRate,
  pub buffer_size: usize,
  pub rpc_port: usize,
  pub webserver_port: usize,
  pub config_path: String
}

#[derive(Debug, PartialEq, Clone)]
pub struct DASPState {
  dasp_status: DASPStatus,
  dasp_proc_type: DASPProcessorType,
}

impl Default for EngineConfig {
  fn default() -> Self {
    let ch_vec = vec![
      ChannelConfig { id: 3, name: "Channel 3".to_string(), permission: ChannelPermissions::USER, is_redundant: true },
      ChannelConfig { id: 4, name: "Channel 4".to_string(), permission: ChannelPermissions::USER, is_redundant: true },
      ChannelConfig { id: 5, name: "Channel 5".to_string(), permission: ChannelPermissions::USER, is_redundant: true },
      ChannelConfig { id: 6, name: "Channel 6".to_string(), permission: ChannelPermissions::USER, is_redundant: true },
      ChannelConfig { id: 7, name: "Channel 7".to_string(), permission: ChannelPermissions::USER, is_redundant: true },
      ChannelConfig { id: 8, name: "Channel 8".to_string(), permission: ChannelPermissions::USER, is_redundant: true },
      ChannelConfig { id: 9, name: "Channel 9".to_string(), permission: ChannelPermissions::USER, is_redundant: true },
      ChannelConfig { id: 10, name: "Channel 10".to_string(), permission: ChannelPermissions::USER, is_redundant: true },
      ChannelConfig { id: 11, name: "Channel 11".to_string(), permission: ChannelPermissions::USER, is_redundant: true },
      ChannelConfig { id: 12, name: "Channel 12".to_string(), permission: ChannelPermissions::USER, is_redundant: true },
      ChannelConfig { id: 13, name: "Channel 13".to_string(), permission: ChannelPermissions::USER, is_redundant: true },
      ChannelConfig { id: 14, name: "Channel 14".to_string(), permission: ChannelPermissions::USER, is_redundant: true },
      ChannelConfig { id: 15, name: "Channel 15".to_string(), permission: ChannelPermissions::USER, is_redundant: true },
      ChannelConfig { id: 16, name: "Channel 16".to_string(), permission: ChannelPermissions::USER, is_redundant: true },
    
    ];
    let buses_vec = vec![
      BusConfig { id: 3, name: "Aux 1".to_string(), permission: ChannelPermissions::USER, is_redundant: false, bus_type: BusType::AUX }
    ];

    Self {
      name: "MixerOS-Engine".to_string(),
      channels: ch_vec,
      role: EngineRole::Controller,
      buses: buses_vec,
      bit_depth: BitDepth::BIT32,
      sample_rate: SampleRate::Hz44100,
      buffer_size: 512,
      rpc_port: 3000,
      webserver_port: 3000,
      config_path: " ".to_string()
    }
  }
}

impl EngineConfig {
  fn new(name: String, channels: Vec<ChannelConfig>, buses: Vec<BusConfig>, bit_depth: BitDepth, sample_rate: SampleRate, buffer_size: usize, port: usize, web_port: usize, role: EngineRole) -> Self {

    Self {
      name,
      channels,
      role,
      buses,
      bit_depth,
      sample_rate,
      buffer_size,
      rpc_port: port,
      webserver_port: web_port,
      config_path: " ".to_string()
    }
  }

  fn set_path(&mut self, path: &Path) {
    self.config_path = path.to_str().expect("Could not save path").to_string()
  }
}

impl Default for DASPState {
  fn default() -> Self {
    Self {
      dasp_status: DASPStatus::STARTING,
      dasp_proc_type: DASPProcessorType::NONE
    }
  }
}

async fn find_file(dir: &Path, target: &str) -> Option<PathBuf> {
  let mut entries = fs::read_dir(dir).await.ok()?;

  while let Ok(Some(entry)) = entries.next_entry().await {
      let path = entry.path();
      let meta = fs::metadata(&path).await.ok()?;
  
      if meta.is_dir() {
          // recurse into subdirectory
          if let Some(found) = Box::pin(find_file(&path, target)).await {
              return Some(found);
          }
      } else if path.file_name().and_then(|n| n.to_str()) == Some(target) {
          return Some(path);
      }
  }

  None
}

impl StateManager {
  pub async fn new() -> Self {
    Self {
      dasp_state: Arc::new(Mutex::new(DASPState::default())),
      config: Arc::new(Mutex::new(EngineConfig::default()))
    }
  }

  pub fn get_config(&mut self) -> Result<Arc<EngineConfig>, StateManagerError> {
    match self.config.lock() {
        Ok(mutex) => Ok(Arc::new(mutex.clone())),
        Err(_) => return Err(StateManagerError::MutexLockError),
    }
  }

  pub fn get_dasp_state(&mut self) -> Result<Arc<DASPState>, StateManagerError> {
    match self.dasp_state.lock() {
        Ok(mutex) => Ok(Arc::new(mutex.clone())),
        Err(_) => return Err(StateManagerError::MutexLockError),
    }
  }

  pub async fn init(&mut self) -> Result<EngineConfig, StateManagerError>{

    if cfg!(target_os = "linux") {
      match find_file(Path::new("/var/MixerOS/Engine/config/"), "engine.yaml").await {
        Some(file) => {
          let path = file.as_path();
          Ok(self.load_config(path).await.ok_or(StateManagerError::FsReadError)?)
        },
        None => Err(StateManagerError::FsReadError),
      }
    } else {
      match find_file(Path::new("./MixerOS"), "engine.yaml").await {
        Some(file) => {
          let path = file.as_path();
          Ok(self.load_config(path).await.ok_or(StateManagerError::FsReadError)?)
        },
        None => Err(StateManagerError::FsReadError),
      }
    }

  }

  pub async fn create_store(&mut self) -> Result<(), std::io::Error> {
    if cfg!(not(target_os = "linux")) {
      let path: PathBuf = ["./"].iter().collect();
      let _ = fs::DirBuilder::new().recursive(true).create(path).await;
    } else {
      let path: PathBuf = ["/var/snap/mixeros-engine/"].iter().collect();
      let _ = fs::DirBuilder::new().recursive(true).create(path).await;
    }

    Ok(())
  }

  pub async fn load_config(&mut self, dir: &Path) -> Option<EngineConfig> {

    match find_file(dir, "config.yaml").await {
        Some(path) => {
          let contents = tokio::fs::read_to_string(path).await.ok()?;
          let config: EngineConfig = yaml_serde::from_str(&contents).ok()?;
          Some(config)
        },
        None => return None
    }
  }

  pub async fn save_config(&mut self, config: &EngineConfig) -> anyhow::Result<()> {
    let contents = yaml_serde::to_string(config)?;
    let path = &self.config.lock().unwrap().config_path;
    
    tokio::fs::write(path, contents).await?;
    Ok(())
  }

}