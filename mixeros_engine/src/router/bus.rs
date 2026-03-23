use jack::{ AudioIn, AudioOut, Unowned, Client, ClientOptions, AsyncClient, ProcessScope, Control };
use jack::Port;
use jack::contrib::ClosureProcessHandler;

use core::slice;
use std::sync::{ Arc, Mutex };


use crate::dasp::module_manager::ModuleManager;
use crate::router::error::BusError;
use crate::system::util::*;
use crate::system::util::{ChannelType, SampleRate};
use crate::dasp::processor::*;
use crate::dasp::dasp_modules::modules::*;
use crate::cli::table::LiveTable;

pub struct Bus {
    name: String,
    status: DASPStatus,
    jack: Option<AsyncClient<(), ClosureProcessHandler<(), Box<dyn FnMut(&Client, &ProcessScope) -> Control + Send>>>>,
    id: usize,
    bus_type: BusType,
    channels: usize,
    level: i8,
    gain: i8,
    mute: bool,
    inputs:  Option<Vec<Arc<Port<Unowned>>>>,
    outputs: Option<Vec<Arc<Mutex<Port<Unowned>>>>>,
    processor: Arc<std::sync::RwLock<ModuleManager>>,
    kernel_manager: Arc<KernelManager>,
}

impl Bus {
    pub fn new(name: String, id: usize, bus_type: BusType, channels: usize, sample_rate: SampleRate, buffer_size: usize, kernel_manager: Arc<KernelManager>) -> Self {
        let manager = ModuleManager::new(buffer_size);

        

        Self {
            name,
            status: DASPStatus::STARTING,
            jack: None,
            id,
            bus_type,
            channels,
            level: 0,
            gain: 0,
            mute: true,
            inputs: None,
            outputs: None,
            processor: Arc::new(std::sync::RwLock::new(manager)),
            kernel_manager,
        }
    }

    pub fn table_row(&mut self) -> Vec<String> {
        let table_row = vec![ self.id.clone().to_string(), self.name.clone(), format!("BUS ({:?})", self.bus_type.clone()), format!("{:?}", self.status)];

        table_row
    }

    pub fn add_modules(&mut self) {
        let eq = Box::new(ParametricEq::new(self.kernel_manager.clone()));

        let _ = self.processor.write().expect("Could not lock processor to add modules").add_module("Parametric EQ", eq);
    }

    pub async fn run(&mut self) -> Result<(), BusError> {
        let mut client_options = ClientOptions::empty();

        client_options.insert(ClientOptions::USE_EXACT_NAME);
        client_options.insert(ClientOptions::NO_START_SERVER);
        
        let (jack, _status) = Client::new(&self.name.as_str(), client_options).unwrap();

        let mut input: Vec<Arc<Port<AudioIn>>> = Vec::new();
        let mut output: Vec<Arc<Mutex<Port<AudioOut>>>> = Vec::new();

        let mut inputs_unowned: Vec<Arc<Port<Unowned>>> = Vec::new();
        let mut outputs_unowned: Vec<Arc<Port<Unowned>>> = Vec::new();

        for channel in 0..self.channels {
            let inp: Port<AudioIn> = jack.register_port(format!("Input {}", channel).as_str(), AudioIn::default()).unwrap();
            let out: Port<AudioOut> = jack.register_port(format!("Output {}", channel).as_str(), AudioOut::default()).unwrap();

            inputs_unowned.push(Arc::new(inp.clone_unowned()));
            outputs_unowned.push(Arc::new(out.clone_unowned()));

            input.push(Arc::new(inp));
            output.push(Arc::new(Mutex::new(out)));
            
        }

        let processor_ptr = Arc::clone(&self.processor);
        let channels: usize = self.channels.clone();

        let closure: Box<dyn FnMut(&Client, &ProcessScope) -> Control + Send> = 
          Box::new( move |_client: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
                let mut processor = processor_ptr.write().unwrap();

                for channel in 0..channels {
                    // Safe read from input buffer
                    let data: Vec<f32> = unsafe {
                        let ptr = input[channel].buffer(ps.n_frames()) as *const f32;
                        slice::from_raw_parts(ptr, ps.n_frames() as usize).to_vec()
                    };

                    let proc_out: Vec<f32> = processor.process_chain_buffer_mono(data);

                    // Guard against length mismatch
                    let out = Arc::clone(&mut output[channel]);
                    let mut out_unlocked = out.lock().unwrap();
                    let out_slice = out_unlocked.as_mut_slice(ps);
                    let n = out_slice.len().min(proc_out.len());
                    out_slice[..n].copy_from_slice(&proc_out[..n]);
                }

                jack::Control::Continue
              });

        let process = jack::contrib::ClosureProcessHandler::new(closure);
        self.jack = Some(jack.activate_async((), process).expect("Jack activate_async Failed"));

        Ok(())
    }

    pub fn get_name(&mut self) -> String {
        return self.name.clone();
    }
    pub fn get_level(&mut self) -> i8 {
        return self.level;
    }
    pub fn get_gain(&mut self) -> i8 {
        return self.gain;
    }
    pub fn get_mute(&mut self) -> bool {
        return self.mute;
    }

    pub fn set_name(&mut self, name: String) -> Result<(), BusError> {
        if name.len() == 0 || name.len() > 30 {
            return Err(BusError::InvalidName);
        }
        self.name = name;
        Ok(())
    }

    pub fn set_level(&mut self, level: i8) -> Result<(), BusError> {
        if level < 10 || level > -99 {
            return Err(BusError::InvalidLevel);
        }
        self.level = level;
        Ok(())
    }

    pub fn set_gain(&mut self, gain: i8) -> Result<(), BusError> {
        if gain < 60 || gain > -20 {
            return Err(BusError::InvalidGain);
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
}
