use mixeros_protocol::mixeros_protocol_sys::proto::{include::util_capnp, *};
use crate::system::state::{ DASPState, EngineConfig, StateManager};
use crate::system::util::{ self, EngineRole };
use sysinfo::{ Components, System };
use crate::engine::Engine;
use crate::system::system;

use std::sync::{ Arc, Mutex };
use std::time;

use tokio::sync::RwLock;

pub struct EngineImpl {
  state_manager: Arc<RwLock<StateManager>>,
  engine_config: Arc<EngineConfig>,
  dasp_state: Arc<DASPState>,
  engine: Arc<RwLock<Engine>>,
  components: Components
}

impl EngineImpl {
  pub fn new(state_manager: Arc<RwLock<StateManager>>, engine: Arc<RwLock<Engine>>) -> Self {
    let state_manager_clone = Arc::clone(&state_manager);
    let components = Components::new_with_refreshed_list();

    Self {
      state_manager: state_manager_clone,
      engine_config: state_manager.blocking_write().get_config().unwrap(),
      dasp_state: state_manager.blocking_write().get_dasp_state().unwrap(),
      engine: Arc::clone(&engine),
      components
    }
  }

  pub fn update_config(&mut self, state_manager: Arc<Mutex<StateManager>>) {
    self.dasp_state = state_manager.lock().unwrap().get_dasp_state().unwrap();
    self.engine_config = state_manager.lock().unwrap().get_config().unwrap();
  }
}

impl engine_capnp::engine::Server for EngineImpl {
  async fn get_role(
    self: capnp::capability::Rc<Self>,
    _: engine_capnp::engine::GetRoleParams<>,
    mut results: engine_capnp::engine::GetRoleResults<>
  ) -> Result<(), capnp::Error> {

    let role: util_capnp::EngineRole = match self.engine_config.role {
        crate::system::util::EngineRole::Controller => util_capnp::EngineRole::Controller,
        crate::system::util::EngineRole::Node => util_capnp::EngineRole::Node,
        crate::system::util::EngineRole::RedundancyController => util_capnp::EngineRole::RedundancyController,
        crate::system::util::EngineRole::RedundancyNode => util_capnp::EngineRole::RedundancyNode,
    };

    results.get().set_role(role);
    Ok(())
  }

  async fn set_role(
    self: capnp::capability::Rc<Self>,
    params: engine_capnp::engine::SetRoleParams<>,
    _: engine_capnp::engine::SetRoleResults<>
  ) -> Result<(), capnp::Error> {
    let role: EngineRole;

    match params.get()?.get_role()? {
      util_capnp::EngineRole::Controller => role = EngineRole::Controller,
      util_capnp::EngineRole::Node => role = EngineRole::Node,
      util_capnp::EngineRole::RedundancyController => role =EngineRole::RedundancyController,
      util_capnp::EngineRole::RedundancyNode => role = EngineRole::RedundancyNode,
    };

    let new: EngineConfig = EngineConfig {role: role, ..self.engine_config.as_ref().clone()};
    let _ = self.state_manager.blocking_write().save_config(&new).await;

    Ok(())
  }
}

impl engine_capnp::d_a_s_p_info::Server for EngineImpl {
  async fn get_c_p_u(
    self: capnp::capability::Rc<Self>,
    _: engine_capnp::d_a_s_p_info::GetCPUParams<>,
    mut results: engine_capnp::d_a_s_p_info::GetCPUResults<>
  ) -> Result<(), capnp::Error> {
    let temp: f32 = self.components.iter().find(|c| c.label().to_lowercase().contains("core 0")).unwrap().temperature().unwrap();
    let data = system::get_cpu(System::new_all(), temp);
    let mut res = results.get().init_cpu();
    
    for (i, cpu) in data.iter().enumerate() {
      
      res.reborrow().init_name(data[i].name.len() as u32);
      res.reborrow().init_vendor(data[i].vendor.len() as u32);
      res.reborrow().init_vendor_id(data[i].vendor_id.len() as u32);

      let _ = &res.set_name(cpu.name.clone());
      let _ = &res.set_vendor(cpu.vendor.clone());
      let _ = &res.set_vendor_id(cpu.vendor_id.clone());
      let _ = &res.set_cores(cpu.cores);
      let _ = &res.set_threads(cpu.threads);
      let _ = &res.set_temp(temp);
      let _ = &res.set_timestamp(cpu.timestamp);
    }

    Ok(()) 
  }
  
  async fn get_num_channels(
    self: ::capnp::capability::Rc<Self>,
    _:engine_capnp::d_a_s_p_info::GetNumChannelsParams<>,
    mut results: engine_capnp::d_a_s_p_info::GetNumChannelsResults<>
  ) -> Result<(), ::capnp::Error> {
    let channels = self.engine.blocking_write().get_channels().len() as u32;
    results.get().set_channels(channels.clone());
    Ok(())
  }
  
  async fn get_num_buses(
    self: capnp::capability::Rc<Self>,
    _:engine_capnp::d_a_s_p_info::GetNumBusesParams<>,
    mut results: engine_capnp::d_a_s_p_info::GetNumBusesResults<>
  ) -> Result<(), capnp::Error> {
    let mut buses = results.get().init_buses();
    let mut aux: u32 = 0;
    let mut group: u32 = 0;
    let mut matrix: u32 = 0;

    for (_key, value) in self.engine.blocking_write().get_buses() {
      if value.read().await.get_type() == util::BusType::AUX {
        aux += 1
      } else if value.read().await.get_type() == util::BusType::GROUP {
        group += 1
      } else if value.read().await.get_type() == util::BusType::AUX {
        matrix += 1
      }
    }

    buses.set_auxes(aux);
    buses.set_groups(group);
    buses.set_matrices(matrix);

    Ok(())
  }

  async fn get_num_dca(
    self: ::capnp::capability::Rc<Self>,
    _:engine_capnp::d_a_s_p_info::GetNumDcaParams<>,
    _:engine_capnp::d_a_s_p_info::GetNumDcaResults<>
  ) -> Result<(), ::capnp::Error> {
    todo!("Implement DCA")
  }
  
  async fn get_num_mute_groups(
    self: ::capnp::capability::Rc<Self>,
    _:engine_capnp::d_a_s_p_info::GetNumMuteGroupsParams<>,
    _:engine_capnp::d_a_s_p_info::GetNumMuteGroupsResults<>
  ) -> Result<(), ::capnp::Error> {
    todo!("Implement Mute groups")
  }
  
  async fn get_memory(
    self: ::capnp::capability::Rc<Self>,
    _:engine_capnp::d_a_s_p_info::GetMemoryParams<>,
    mut results: engine_capnp::d_a_s_p_info::GetMemoryResults<>
  ) -> Result<(), ::capnp::Error> {
    let data = system::get_memory(System::new_all());

    let mut mem = results.get().init_memory();

    mem.set_heap_total(data.heap_total);
    mem.set_heap_used(data.heap_used);
    mem.set_memory_total(data.mem_total);
    mem.set_memory_used(data.mem_used);
    mem.set_timestamp(data.timestamp);
    Ok(()) 
  }
       
  
  async fn get_processor(
    self: ::capnp::capability::Rc<Self>,
    _:engine_capnp::d_a_s_p_info::GetProcessorParams<>,
    _:engine_capnp::d_a_s_p_info::GetProcessorResults<>
  ) -> Result<(), ::capnp::Error> {
    todo!("Implement Processor Metrics")
  }
  
  async fn get_uptime(
    self: ::capnp::capability::Rc<Self>,
    _:engine_capnp::d_a_s_p_info::GetUptimeParams<>,
    mut results: engine_capnp::d_a_s_p_info::GetUptimeResults<>
  ) -> Result<(), ::capnp::Error> {
    let uptime = time::Instant::now().elapsed().as_secs();
    results.get().set_uptime(uptime);

    Ok(())
  }
  
  async fn get_clock_data(
    self: ::capnp::capability::Rc<Self>,
    _:engine_capnp::d_a_s_p_info::GetClockDataParams<>,
    _:engine_capnp::d_a_s_p_info::GetClockDataResults<>
  ) -> Result<(), ::capnp::Error> {
      todo!("Implement Processor Metrics")
  }
  
  async fn get_network(
      self: ::capnp::capability::Rc<Self>,
      _: engine_capnp::d_a_s_p_info::GetNetworkParams<>,
      mut results: engine_capnp::d_a_s_p_info::GetNetworkResults<>,
  ) -> Result<(), ::capnp::Error> {
    let data = system::get_network(sysinfo::Networks::new_with_refreshed_list());

    let mut interfaces = results.get().init_interfaces(data.len() as u32);

    for (i, iface) in data.iter().enumerate() {
        let mut entry = interfaces.reborrow().get(i as u32);

        entry.reborrow().set_name(&iface.name);
        entry.reborrow().set_status(iface.status);

        {
            let mut mac = entry.reborrow().init_mac();
            mac.set_oui0(iface.mac.oui0);
            mac.set_oui1(iface.mac.oui1);
            mac.set_oui2(iface.mac.oui2);
            mac.set_nic0(iface.mac.nic0);
            mac.set_nic1(iface.mac.nic1);
            mac.set_nic2(iface.mac.nic2);
        }

        {
            let mut v4 = entry.reborrow().init_ipv4(iface.ipv4.len() as u32);
            for (j, ip) in iface.ipv4.iter().enumerate() {
                let addr = ip.octets();
                let mut slot = v4.reborrow().get(j as u32);
                slot.set_group0(addr[0]);
                slot.set_group1(addr[1]);
                slot.set_group2(addr[2]);
                slot.set_group3(addr[3]);
            }
        }

        {
            let mut v6 = entry.reborrow().init_ipv6(iface.ipv6.len() as u32);
            for (j, ip) in iface.ipv6.iter().enumerate() {
                let addr = ip.octets();
                let mut slot = v6.reborrow().get(j as u32);
                slot.set_group0(u16::from_be_bytes([addr[0],  addr[1]]));
                slot.set_group1(u16::from_be_bytes([addr[2],  addr[3]]));
                slot.set_group2(u16::from_be_bytes([addr[4],  addr[5]]));
                slot.set_group3(u16::from_be_bytes([addr[6],  addr[7]]));
                slot.set_group4(u16::from_be_bytes([addr[8],  addr[9]]));
                slot.set_group5(u16::from_be_bytes([addr[10], addr[11]]));
                slot.set_group6(u16::from_be_bytes([addr[12], addr[13]]));
                slot.set_group7(u16::from_be_bytes([addr[14], addr[15]]));
            }
        }
    }

    Ok(())
  }
  
  async fn get_num_groups(
    self: ::capnp::capability::Rc<Self>,
    _:engine_capnp::d_a_s_p_info::GetNumGroupsParams<>,
    _:engine_capnp::d_a_s_p_info::GetNumGroupsResults<>
  ) -> Result<(), ::capnp::Error>  {
    todo!("Implement Groupsb")  
  }
}

