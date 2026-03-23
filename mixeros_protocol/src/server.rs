use tokio::net::*;
use mdns_sd::{ ServiceDaemon, ServiceInfo };

use std::collections::HashMap;

use super::client::*;

pub enum ServerError {

}

pub struct Server {
    name: String,
    addr: String,
    port: u16,
    clients: HashMap<usize, Client>
}

impl Server {
    pub fn new(name: String, addr: String, port: u16) -> Self {
        let clients: HashMap<usize, Client> = HashMap::new();

        Self {
            name,
            addr,
            port,
            clients
        }
    }

    pub fn run_service_discovery(&mut self) {
        let mdns = ServiceDaemon::new().unwrap();
        let service_type: &str = "_mixeros-engine._tcp.local.";

        let info = ServiceInfo::new(
            service_type,
            &self.name,               
            &format!("{}.local.", &self.name),
            (),                 
            self.port,
            None,               
        ).unwrap();

        mdns.register(info).unwrap();

    }

    pub fn handshake(&mut self) -> Result<(), ServerError> {
        

        Ok(())
    }
    
}