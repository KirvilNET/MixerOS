use std::time;

use axum::{ Router, http::Response, routing::{get} };

use serde_json;
use serde::{ Deserialize, Serialize };

use sysinfo::{ Networks, System };

#[derive(Serialize, Deserialize)]
struct CPUData {
  name: String, 
  value: f32
}

#[derive(Serialize, Deserialize)]
struct CPUDataArray {
  pub timestamp: u64,
  pub cpu: Vec<CPUData>
}

#[derive(Serialize, Deserialize)]
struct Memory {
  pub timestamp: u64,
  pub used_mem: u64,
  pub avalible_mem: u64,
  pub used_swap: u64,
  pub avalible_swap: u64,
  pub total_mem: u64,
  pub total_swap: u64
}

#[derive(Serialize, Deserialize)]
struct NetworkInterface {
  pub name: String,
  pub ip: Vec<String>,
  pub mac: String,
}

pub struct SystemMetrics;

impl SystemMetrics {
  pub fn get_cpu() -> Response<String> { // Web: {timestamp: number, cpu: Array<{name: string, value: number}> }
    let mut sys = System::new();
    
    sys.refresh_cpu_all();

    let time = time::SystemTime::now().elapsed().unwrap().as_secs();

    let mut dataset: Vec<CPUData> = Vec::new();
    
    for cpu in sys.cpus() {
      let name = cpu.name();
      let value = cpu.cpu_usage();

      let data = CPUData { name: name.to_string(), value };
      dataset.push(data);
    }

    let res = Response::builder().status(200).header("Content-Type", "text/json");

    return res.body(serde_json::to_string(&CPUDataArray { timestamp: time, cpu: dataset }).expect("Could not get CPU data")).unwrap()
  }

  pub fn get_memory() -> Response<String> {
    let mut sys = System::new();

    sys.refresh_memory();

    let time = time::SystemTime::now().elapsed().unwrap().as_secs();

    let avalible_mem = sys.free_memory();
    let avalible_swap = sys.free_swap();

    let used_mem = sys.used_memory();
    let used_swap = sys.used_swap();

    let total_mem = avalible_mem + used_mem;
    let total_swap = avalible_swap + used_swap;

    let res = Response::builder().status(200).header("Content-Type", "text/json");
    

    return res.body(serde_json::to_string(&Memory {timestamp: time, used_mem, avalible_mem, used_swap, avalible_swap, total_mem, total_swap}).expect("Could not get Memory data")).unwrap()
  }

  pub fn get_net() -> Response<String> {
    let res = Response::builder().status(200).header("Content-Type", "text/json");
    let mut net = Networks::new();

    net.refresh(true);

    let mut interfaces: Vec<NetworkInterface> = Vec::new();

    for (interface_name, network) in &net {
      let ip: Vec<String> = network.ip_networks().iter().map(|i| i.to_string()).collect();
      let mac: String = network.mac_address().to_string();
      
      let interface = NetworkInterface { name: interface_name.to_string(), ip, mac };

      interfaces.push(interface);
    }

    return res.body(serde_json::to_string(&interfaces).expect("Could not get Network data")).unwrap()
  }

}

pub fn get_router() -> Router<()> {

  let router = Router::new()
    .route("/cpu", get(SystemMetrics::get_cpu()))
    .route("/memory", get(SystemMetrics::get_memory()))
    .route("/network", get(SystemMetrics::get_net()));
  router
}


