use jack::contrib::ClosureProcessHandler;
use tokio::sync::{ RwLock };

use core::slice;
use std::sync::Arc;

use jack::{ AudioIn, AudioOut, Client, ClientOptions, AsyncClient, ProcessScope, Control };
use jack::Port;

use crate::system::util::{ ChannelType, SampleRate };
use crate::dasp::module_manager::ModuleManager;
use crate::router::error::ChannelStripError;
use crate::system::util::*;
use crate::dasp::processor::processor::*;
use crate::dasp::dasp_modules::modules::{ ParametricEq };

pub struct ChannelStrip {
  name: String,
  jack: Option<AsyncClient<(), ClosureProcessHandler<(), Box<dyn FnMut(&Client, &ProcessScope) -> Control + Send>>>>,
  status: DASPStatus,
  id: usize,
  ch_type: ChannelType,
  level: i8,
  gain: i8,
  mute: bool,
  input: Option<Arc<Port<AudioIn>>>,
  output: Option<Arc<Port<AudioOut>>>,
  processor: Arc<std::sync::RwLock<ModuleManager>>
}

impl ChannelStrip {
  pub fn new(name: String, id: usize, ch_type: ChannelType, sample_rate: SampleRate, buffer_size: u32, proc: Processor) -> Self {
    let manager = ModuleManager::new(proc);

    Self {
      name,
      jack: None,
      status: DASPStatus::STARTING,
      id,
      ch_type,
      level: 0,
      gain: 0,
      mute: true,
      input: None,
      output: None,
      processor: Arc::new(std::sync::RwLock::new(manager))
    }
  }

  fn add_modules(&mut self) {
    let eq = Box::new(ParametricEq::new());

    let _ = self.processor.write().expect("Could not lock processor to add modules").add_processor("Parametric EQ", eq);
  }

  pub async fn run(&mut self) -> Result<(), ChannelStripError> {
      let client_options = { ClientOptions::USE_EXACT_NAME };
      let (jack, _status) = Client::new(&self.name, client_options).unwrap();

      let input: Port<AudioIn> = jack.register_port("Input", AudioIn::default()).unwrap();
      let mut output: Port<AudioOut> = jack.register_port("Output", AudioOut::default()).unwrap();

      let processor_ptr = Arc::clone(&self.processor);

      let closure: Box<dyn FnMut(&Client, &ProcessScope) -> Control + Send> = 
        Box::new( move |_client: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
              let mut processor = processor_ptr.write().unwrap();

              // Safe read from input buffer
              let data: Vec<f32> = unsafe {
                  let ptr = input.buffer(ps.n_frames()) as *const f32;
                  slice::from_raw_parts(ptr, ps.n_frames() as usize).to_vec()
              };

              let proc_out: Vec<f32> = processor.process_chain_buffer_mono(data);

              // Guard against length mismatch
              let out_slice = output.as_mut_slice(ps);
              let n = out_slice.len().min(proc_out.len());
              out_slice[..n].copy_from_slice(&proc_out[..n]);

              jack::Control::Continue
            });

      let process = jack::contrib::ClosureProcessHandler::new(closure);

      self.jack = Some(jack.activate_async((), process).unwrap());

      Ok(())
  }

  pub fn get_name(&mut self) -> String { return self.name.clone(); }
  pub fn get_level(&mut self) -> i8 { return self.level; }
  pub fn get_gain(&mut self) -> i8 { return self.gain; }
  pub fn get_mute(&mut self) -> bool { return self.mute; }
  pub fn get_type(&mut self) -> ChannelType { return self.ch_type }
  pub fn get_id(&mut self) -> usize { return self.id }

  pub fn set_name(&mut self, name: String) -> Result<(), ChannelStripError> { 
    if name.len() == 0 || name.len() > 30 {
      return Err(ChannelStripError::InvalidName)
    }
    self.name = name;
    Ok(())
  }

  pub fn set_level(&mut self, level: i8) -> Result<(), ChannelStripError> { 
    if level < 10 || level > -99 {
      return Err(ChannelStripError::InvalidLevel)
    }
    self.level = level;
    Ok(())
  }

  pub fn set_gain(&mut self, gain: i8) -> Result<(), ChannelStripError> { 
    if gain < 60 || gain > -20 {
      return Err(ChannelStripError::InvalidGain)
    }
    self.gain = gain;
    Ok(())
  }

  pub fn set_mute(&mut self, mute: bool) { 
    if self.mute != mute {
      self.mute = mute;
    }
  } 

  pub fn set_status(&mut self, status: DASPStatus) {
    self.status = status;
  }

  pub fn set_id(&mut self, id: usize) -> Result<(), ChannelStripError> {
    if self.ch_type == ChannelType::USER {
      self.id = id;
      return Ok(())
    } else {
      return Err(ChannelStripError::SystemChannel)
    }
  }
}