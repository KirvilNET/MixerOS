use tokio::net::*;
use mdns_sd::{ ScopedIp, ServiceDaemon, ServiceEvent };

use std::collections::{HashMap, HashSet};

use super::util::*;

pub enum ClientError {

}

pub struct Client {
    client_name: String,
    server_name: String,
    tcp: Option<TcpStream>,
    port: u16,
}

pub struct DiscoveredServer {
    name: String,
    ip: String,
    tcp: TcpStream,
    port: u16,
}

impl Client {

    pub async fn browse_mdns(service_type: ServiceType) {

        let service: &str = match service_type {
            ServiceType::Engine => "_mixeros-engine._tcp.local.",
            ServiceType::StageBox => "_mixeros-stagebox._tcp.local.",
        };

        let mdns = ServiceDaemon::new().unwrap();
        let receiver = mdns.browse(service).unwrap();

        let mut connected: HashMap<String, tokio::task::JoinHandle<()>> = HashMap::new();

        while let Ok(event) = receiver.recv_async().await {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    let name = info.get_fullname().to_string();
                    
                    if connected.contains_key(&name) {
                        continue;
                    }

                    let Some(ip) = info.get_addresses().iter().next() else {
                        eprintln!("Resolved {name} but got no IP, skipping");
                        continue;
                    };
                    
                    //let handle = tokio::spawn();
                    //connected.insert(name, handle);
                }
                ServiceEvent::ServiceRemoved(_, name) => {
                    println!("Service went offline: {name}");
                }
                _ => {}
            }
        }

    }

    pub fn connect() {

    }

}