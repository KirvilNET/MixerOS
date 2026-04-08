use std::{net::{IpAddr, Ipv6Addr, Ipv4Addr}, time::UNIX_EPOCH};

use sysinfo::{ Components, Networks, System };
use getifs::{ Flags, interfaces };

pub struct CPU {
  pub name: String,
  pub vendor: String,
  pub vendor_id: String,
  pub cores: u8,
  pub threads: u16,
  pub temp: f32,
  pub usage: f32,
  pub timestamp: i64
}

pub struct Memory {
  pub mem_total: u32,
  pub mem_used: u32,
  pub heap_total: u32,
  pub heap_used: u32,
  pub timestamp: i64
}

pub struct MAC {
  pub oui0: u8,
  pub oui1: u8,
  pub oui2: u8,
  pub nic0: u8,
  pub nic1: u8,
  pub nic2: u8,
}

pub struct SubnetMask {
  pub group0: u8, 
  pub group1: u8,
  pub group2: u8,
  pub group3: u8,
}

pub struct Interface {
  pub name: String,
  pub status: bool,
  pub mac: MAC,
  pub ipv4: Vec<Ipv4Addr>,
  pub ipv6: Vec<Ipv6Addr>,
}
 
pub fn get_cpu(mut sys: System, temp: f32) -> Vec<CPU> {
  sys.refresh_cpu_all();

  let mut cpu_vec: Vec<CPU> = Vec::new();
  let cores: u8 = std::thread::available_parallelism().unwrap().get() as u8;
  let timestamp = std::time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64() as i64;

  for cpu in sys.cpus() {
    let cpu_data = CPU {
      name: cpu.name().to_string(),
      vendor: cpu.brand().to_string(),
      vendor_id: cpu.vendor_id().to_string(),
      cores,
      threads: (cores * 2) as u16,
      temp,
      usage: cpu.cpu_usage(),
      timestamp 
    };

    cpu_vec.push(cpu_data);
  }

  return cpu_vec
}

pub fn get_memory(mut sys: System) -> Memory {
  sys.refresh_memory();
  let timestamp = std::time::SystemTime::now()
    .duration_since(UNIX_EPOCH).unwrap()
    .as_secs_f64() as i64;

  return Memory {
    mem_total: (sys.total_memory() / 8) as u32,
    mem_used: (sys.used_memory() / 8) as u32,
    heap_total: (sys.total_swap() / 8) as u32,
    heap_used: (sys.used_swap() / 8) as u32,
    timestamp
  }
}

pub fn get_network(mut net: Networks) -> Vec<Interface> {
  net.refresh(true);
  let mut interfaces: Vec<Interface> = Vec::new();

  let mut ipv4: Vec<Ipv4Addr> = Vec::new();
  let mut ipv6: Vec<Ipv6Addr> = Vec::new();
  
  for (interface_name, interface_data) in net.list() {
    let mac_addr = interface_data.mac_address().0;
    let mac =  MAC {
      oui0: mac_addr[0],
      oui1: mac_addr[1],
      oui2: mac_addr[2],
      nic0: mac_addr[3],
      nic1: mac_addr[4],
      nic2: mac_addr[5],
    };

    for network in interface_data.ip_networks() {
      if network.addr.is_ipv4() {
        if let IpAddr::V4(addr) = network.addr {
          ipv4.push(addr);
        }
      } else {
        if let IpAddr::V6(addr) = network.addr {
          ipv6.push(addr)
        }
      }
    }

    let curr = Interface {
      name: interface_name.to_string(),
      status: true,
      mac,
      ipv4: ipv4.clone(),
      ipv6: ipv6.clone(),
    };

    interfaces.push(curr);
  }

  return interfaces
}