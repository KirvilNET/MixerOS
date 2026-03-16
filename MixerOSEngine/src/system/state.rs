use serde::{Deserialize, Serialize};
use tokio::{ fs };

use std::{path::{Path, PathBuf}, sync::*};

use super::util::*;

pub struct StateManager {
  dasp_state: Arc<Mutex<DASPState>>,
  config: Arc<Mutex<EngineConfig>>
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct EngineConfig {
  pub name: String,
  pub channels: usize,
  pub bus: usize,
  pub bit_depth: BitDepth,
  pub sample_rate: SampleRate,
  pub buffer_size: usize,
  pub ws_port: usize,
  config_path: String
}

pub struct DASPState {
  dasp_status: DASPStatus,
  dasp_proc_type: DASPProcessorType,
}

impl Default for EngineConfig {
  fn default() -> Self {
    Self {
      name: "MixerOS-Engine".to_string(),
      channels: 16,
      bus: 2,
      bit_depth: BitDepth::BIT32,
      sample_rate: SampleRate::Hz48000,
      buffer_size: 1024,
      ws_port: 3000,
      config_path: " ".to_string()
    }
  }
}

impl EngineConfig {
  fn new(name: String, channels: usize, bus: usize, bit_depth: BitDepth, sample_rate: SampleRate, buffer_size: usize, port: usize) -> Self {

    Self {
      name,
      channels,
      bus,
      bit_depth,
      sample_rate,
      buffer_size,
      ws_port: port,
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

pub enum StateManagerError {
  FsReadError,
  FsWriteError,
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

  pub async fn save_config(config: &EngineConfig, path: &Path) -> anyhow::Result<()> {
      let contents = yaml_serde::to_string(config)?;
      tokio::fs::write(path, contents).await?;
      Ok(())
  }

}