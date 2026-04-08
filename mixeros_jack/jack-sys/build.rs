use pkg_config::Library;

fn main() {
    let jack_version = std::env::var_os("JACK_VERSION");
    let jack_lib_path = std::env::var_os("JACK_LIB_PATH");
    let _library_found: Library;

    if let Some(path) = jack_lib_path {
        if let Some(ver) = jack_version {
            println!("cargo:warning=Using the path specified in $JACK_LIB_PATH ({})", path.to_str().unwrap());
            _library_found = pkg_config::Config::new().arg(format!("--with-path={}", path.to_str().unwrap()))
            .atleast_version(ver.to_str().unwrap())
            .probe("jack")
            .expect(format!("Couldn't find jack lib at the specified version ({})", ver.into_string().unwrap()).as_str());
        } else {
            println!("cargo:warning=No minimum version of jack specified. Solve this by defining the JACK_VERSION envrioment varible");
            _library_found = pkg_config::Config::new()
            .probe("jack")
            .expect(format!("Couldn't find jack lib", ).as_str());
        }
    } else {
        _library_found = pkg_config::Config::new()
        .probe("jack")
        .expect(format!("Couldn't find jack lib", ).as_str());
    }
}
