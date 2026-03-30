use std::{ collections::HashMap, env, fs, path::{Path, PathBuf}, io::{ Write }};

const COPY_DIR_FOLDER: &'static str = "protocol";

fn main() {
  let out_dir: std::ffi::OsString = env::var_os("OUT_DIR").unwrap();

  let source: String = env::var("CARGO_MANIFEST_DIR").unwrap();
  let copy_dir_path: PathBuf = PathBuf::from(format!("{}/{}", source, COPY_DIR_FOLDER ));
  
  let capnp_path = std::process::Command::new("which").arg("capnp").output().expect("failed to run which").stdout;
  let capnp_path = std::str::from_utf8(&capnp_path).unwrap().trim().to_string();
  let path = env::var("PATH").unwrap_or_default();
  let new_path = format!("{}:{}", path, capnp_path);
  unsafe { std::env::set_var("PATH", &new_path); }

  println!("Out Directory: {}", out_dir.display());
  println!("Source directory: {}", source);
  

  if PathBuf::from(out_dir.clone().into_string().unwrap()).exists() {
    println!("Cleaning previous build");
    fs::remove_dir_all(&out_dir).unwrap();
    fs::create_dir_all(&out_dir).unwrap();
    fs::create_dir_all(format!("{}/compiled", &out_dir.display())).unwrap();
    copy_dir(&copy_dir_path, format!("{}/proto", out_dir.clone().into_string().unwrap()));
  } else {
    fs::create_dir_all(&out_dir).unwrap();
    copy_dir(&copy_dir_path, &out_dir);
  }
  println!("{:?}", out_dir);
  std::env::set_current_dir(PathBuf::from(out_dir.clone()).as_path()).unwrap();

  //let out_string = out.to_str().unwrap().to_string();

  let capnproto_files = get_files(&PathBuf::from(format!("{}/proto", out_dir.clone().into_string().unwrap()))).unwrap();
  let mut compiler = capnpc::CompilerCommand::new();

  let mut dirs: Vec<String> = Vec::new();

  for dir in capnproto_files.keys() {
    let files: Vec<String> = capnproto_files.get(dir).unwrap().to_vec();
    dirs.push(dir.to_string());

    for proto_file in files {

      let stem: PathBuf;
      match PathBuf::from(&proto_file).to_string_lossy().to_string().strip_prefix(&out_dir.clone().into_string().unwrap()) {
        Some(x) => stem = PathBuf::from(x),
        None => stem = PathBuf::from(proto_file),
      }

      let rust_path_def = format!("{}/{}", PathBuf::from(dir).strip_prefix(&out_dir).unwrap().display(), stem.to_string_lossy().to_string());
      compiler.file(PathBuf::from(rust_path_def));
      //fs::File::create(format!("compiled/{}/{}_capnp.rs", PathBuf::from(dir).strip_prefix(&out_dir).unwrap().display(), stem.clone().file_name().unwrap().to_string_lossy().to_string())).unwrap();
    }
    
  }

  println!("Compiling {} files", capnproto_files.values().map(|v| v.len()).sum::<usize>());
  
  //compiler.output_path(format!("compiled")).import_path(include).run().unwrap();
  for include_folder in capnproto_files.keys() {
    compiler.import_path(include_folder);
  }

  println!("cargo:warning=cwd: {:?}", std::env::current_dir());
  compiler.output_path("compiled").src_prefix(out_dir.clone()).run().unwrap();

  println!("Finished Compiling the Captin Proto Files. ");
  println!("Generating mod.rs files for each directory ");

  let mut all_files: Vec<String> = Vec::new();
  for k in capnproto_files.keys() {
    let mut vec = capnproto_files.clone().get(k).unwrap().to_vec();
    all_files.append(vec.as_mut())
  }


  for dir in dirs {
    create_mod(out_dir.as_os_str().to_str().unwrap(), Path::new(&dir), capnproto_files.get(&dir).unwrap(), &all_files.clone(), 0);
  }

  fs::write(format!("{}/genarated_files.rs", out_dir.display()), "#[path = \"compiled/proto/mod.rs\"] \npub mod proto;").unwrap();
  
}

fn create_mod(out_dir: &str, dir: &Path, files: &[String], all_files: &[String], level: usize) {
    // Strip out_dir prefix to get relative path
    let stem = PathBuf::from(
        dir.to_string_lossy()
            .strip_prefix(out_dir)
            .unwrap_or(&dir.to_string_lossy())
            .trim_start_matches('/')
    );

    let compiled_dir = PathBuf::from("compiled").join(&stem);
    println!("cargo:warning=create_mod: stem={} compiled_dir={}", stem.display(), compiled_dir.display());

    if !compiled_dir.exists() {
        println!("cargo:warning=create_mod: compiled dir does not exist: {}", compiled_dir.display());
        return;
    }

    let mod_file_path = compiled_dir.join("mod.rs");
    let mod_file = fs::File::create(&mod_file_path).unwrap();
    let mut writer = std::io::BufWriter::new(mod_file);

    // Write capnp file modules
    for file in files {
        let file_stem = PathBuf::from(file)
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .to_string();

        // Get absolute path to the generated .rs file
        let rs_file = compiled_dir.join(format!("{}_capnp.rs", file_stem));
        

        println!("cargo:warning=create_mod: Adding {} to mod file", file_stem);
        writeln!(
            writer,
            "capnp::generated_code!(pub mod {}_capnp, \"{}\");",
            file_stem,
            rs_file.to_string_lossy().replace('\\', "/")
        ).unwrap();
        writeln!(writer).unwrap();
    }

    // Write subdir modules
    for entry in fs::read_dir(&compiled_dir).unwrap().flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().unwrap().to_string_lossy().to_string();
            println!("cargo:warning=create_mod: Found nested module {}", name);
            writeln!(writer, "pub mod {};", name).unwrap();

            // Get actual .capnp derived files in this subdir
            let nested_files: Vec<String> = fs::read_dir(&path)
                .unwrap()
                .flatten()
                .filter(|e| {
                    let p = e.path();
                    p.is_file() && p.extension().map_or(false, |ext| ext == "rs")
                        && p.file_name()
                            .map_or(false, |n| n.to_string_lossy().ends_with("_capnp.rs"))
                })
                .map(|e| {
                    // Strip the _capnp.rs suffix to get the base name
                    let fname = e.path().file_stem().unwrap().to_string_lossy().to_string();
                    fname.trim_end_matches("_capnp").to_string()
                })
                .collect();
            let new_level = level + 1;
            create_mod(out_dir, &path, &nested_files, all_files, new_level + 1);
        }
    }

    println!("cargo:warning=create_mod: Wrote mod.rs to {}", mod_file_path.display());
}

fn copy_dir<P, Q>(from: P, to: Q)
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let to = to.as_ref().to_path_buf();

    for path in fs::read_dir(from).unwrap() {
        let path = path.unwrap().path();
        let to = to.clone().join(path.file_name().unwrap());

        if path.is_file() {
            fs::copy(&path, to).unwrap();
        } else if path.is_dir() {
            if !to.exists() {
                fs::create_dir_all(&to).unwrap();
            }

            copy_dir(&path, to);
        } else {}
    }
}

fn get_files(dir: &Path) -> Result<HashMap<String, Vec<String>>, std::io::Error> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    let mut current_dir_files: Vec<String> = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            if path.extension().unwrap() == "capnp" {
                println!("Found: {:?}", path);
                current_dir_files.push(path.file_name().unwrap().to_string_lossy().to_string());
            }
        } else if path.is_dir() {
            match get_files(&path) {
                Ok(sub) => {
                    for (sub_dir, sub_files) in sub {
                        map.entry(sub_dir)
                            .or_insert(sub_files);
                    }
                }
                Err(err) => println!("Error reading {:?}: {}", path, err),
            }
        }
    }

    if !current_dir_files.is_empty() {
        map.entry(dir.to_string_lossy().to_string())
            .or_default()
            .extend(current_dir_files);
    }

    Ok(map)
}