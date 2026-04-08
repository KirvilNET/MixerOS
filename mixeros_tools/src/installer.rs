use std::path::{Path, PathBuf};
use std::process::{ Command };

use git2::{ Repository };

const REPO_URL: &'static str = "https://github.com/KirvilNET/MixerOS";

#[derive(Debug)]
pub enum InstallerError {
  GitError(git2::Error),
  DownloadError(i32)
}

fn get_latest() -> String {
    // git tag | sort -V | tail -1
    let latest = Command::new("git")
      .arg("tag")
      .arg("|")
      .arg("sort")
      .arg("-V")
      .arg("|")
      .arg("tail")
      .arg("1")
      .spawn()
      .unwrap();

    return String::from_utf8(latest.wait_with_output().unwrap().stdout).expect("Couldn't get latest version");
}

fn build(features: Option<super::Features>) -> Result<(), InstallerError> {
  match features {
    Some(f) => {
      match f {
        crate::Features::Engine => {
          let engine_build = Command::new("cargo")
            .arg("build")
            .arg("--package")
            .arg("mixeros_engine")
            .arg("--release")
            .arg("--target-dir")
            .arg("/opt/mixeros/")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

          if engine_build.success() {
            println!("Built the MixerOS Engine")
          } else {
            eprintln!("Failed to build MixerOS Engine")
          }
        },
        crate::Features::Ui => {
          let engine_build = Command::new("cargo")
            .current_dir(Path::new("./MixerOS"))
            .arg("tauri")
            .arg("build")
            .arg("--release")
            .arg("--target-dir")
            .arg("/opt/mixeros/")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

          if engine_build.success() {
            println!("Built the MixerOS Engine")
          } else {
            eprintln!("Failed to build MixerOS Engine")
          }
        },
        crate::Features::Full => {
          let engine_build = Command::new("cargo")
            .arg("build")
            .arg("--package")
            .arg("mixeros_engine")
            .arg("--release")
            .arg("--target-dir")
            .arg("/opt/mixeros/")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

          if engine_build.success() {
            println!("Built the MixerOS Engine")
          } else {
            eprintln!("Failed to build MixerOS Engine")
          }

          let ui_build = Command::new("cargo")
            .current_dir(Path::new("./MixerOS"))
            .arg("tauri")
            .arg("build")
            .arg("--release")
            .arg("--target-dir")
            .arg("/opt/mixeros/")
            .spawn()
            .unwrap()
            .wait()
            .unwrap();

          if ui_build.success() {
            println!("Built the MixerOS UI")
          } else {
            eprintln!("Failed to build MixerOS UI")
          }
        },
      }
    },
    None => eprintln!("No build type selected"),
  }

  Ok(())
}

pub fn install(
  source: Option<String>, 
  dir: Option<String>, 
  version: Option<String>, 
  feature: Option<super::Features>
) -> Result<(), InstallerError> {
  let repo: Repository;

  if let Some(source_path) = source {
    repo = match Repository::open(&source_path) {
      Ok(r) => r,
      Err(err) => return Err(InstallerError::GitError(err)),
    };

    std::env::set_current_dir(&source_path).unwrap();

    build(feature).expect("Failed to build");
    
  } else {
    let path = PathBuf::from(format!("{}/tags/{}", REPO_URL, version.unwrap_or(get_latest())));
    let download = Command::new("wget")
      .arg("-P")
      .arg(dir.clone().unwrap_or("/opt/mixeros/source/".to_string()))
      .arg(path)
      .spawn()
      .unwrap()
      .wait()
      .expect("Couldn't download from the github repo");
    
    if !download.success() {
      return Err(InstallerError::DownloadError(download.code().unwrap()))
    }

    std::env::set_current_dir(dir.clone().unwrap_or("/opt/mixeros/source/".to_string())).unwrap();

    build(feature).expect("Failed to build");
  }

  Ok(())
}