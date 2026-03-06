use serde::{Deserialize, Serialize};
use tokio::{ fs, io };

use std::{path::{Path, PathBuf}, sync::*};

use super::util::*;

pub struct StateManager {
  dasp_state: Arc<Mutex<DASPState>>,
  config: Arc<Mutex<EngineConfig>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EngineConfig {
  pub name: String,
  pub channels: usize,
  pub bus: usize,
  pub bit_depth: BitDepth,
  pub sample_rate: SampleRate,
  pub config_path: String
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
      config_path: " ".to_string()
    }
  }
}

impl EngineConfig {
  fn new(name: String, channels: usize, bus: usize, bit_depth: BitDepth, sample_rate: SampleRate ) -> Self {
    Self {
      name,
      channels,
      bus,
      bit_depth,
      sample_rate,
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
  pub fn new() -> Self {
    
    Self {
      dasp_state: Arc::new(Mutex::new(DASPState::default())),
      config: Arc::new(Mutex::new(EngineConfig::default()))
    }
  }

  pub async fn CreateStore(&mut self) -> Result<(), std::io::Error> {
    let paths: PathBuf = Default::default();

    if cfg!(not(target_os = "linux")) {
      let paths: PathBuf = ["./"].iter().collect();
    } else {
      let paths: PathBuf = ["/var/snap/mixeros-engine/"].iter().collect();

      for path in &paths {
        if fs::try_exists(path).await? == true {
          print!("found {:?}", path)
        } else {
          return Err(std::io::Error::new(std::io::ErrorKind::AddrNotAvailable, "Could Not Find Dir"))
        }
      }
    }
    
    for path in &paths {
      fs::DirBuilder::new().recursive(true).create(path);
    }

    Ok(())
  }

  pub async fn load_config(&mut self) -> Option<EngineConfig> {
    let dir: &Path;

    if cfg!(not(target_os = "linux")) {
      dir = Path::new("./mixeros-engine");
    } else {
      dir = Path::new("/var/snap/mixeros-engine");
    }

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