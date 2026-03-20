use jack::{ AudioIn, AudioOut, Client, ClientOptions, AsyncClient, ProcessScope, Control };
use jack::Port;
use jack::contrib::ClosureProcessHandler;

use core::slice;
use std::sync::Arc;


use crate::dasp::module_manager::ModuleManager;
use crate::router::error::BusError;
use crate::system::util::*;
use crate::system::util::{ChannelType, SampleRate};
use crate::dasp::processor::processor::*;

pub struct Bus {
    name: String,
    status: DASPStatus,
    jack: Option<AsyncClient<(), ClosureProcessHandler<(), Box<dyn FnMut(&Client, &ProcessScope) -> Control + Send>>>>,
    id: usize,
    ch_type: ChannelType,
    level: i8,
    gain: i8,
    mute: bool,
    input: Option<Port<AudioIn>>,
    output: Option<Port<AudioOut>>,
    processor: Arc<std::sync::RwLock<ModuleManager>>,
}

impl Bus {
    pub fn new(name: String, id: usize, ch_type: ChannelType, sample_rate: SampleRate, buffer_size: u32, proc: Processor) -> Self {
        let manager = ModuleManager::new(proc);

        Self {
            name,
            status: DASPStatus::STARTING,
            jack: None,
            id,
            ch_type,
            level: 0,
            gain: 0,
            mute: true,
            input: None,
            output: None,
            processor: Arc::new(std::sync::RwLock::new(manager)),
        }
    }

    pub async fn run(&mut self) -> Result<(), BusError> {
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
