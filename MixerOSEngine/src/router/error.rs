#[derive(Debug)]

pub enum ChannelStripError {
  InvalidName,
  InvalidLevel,
  InvalidGain,
  InvalidInput,
  InvalidOutput,
  StreamError
}

pub enum BusError {
  InvalidName,
  InvalidLevel,
  InvalidGain,
  InvalidInput,
  InvalidOutput,
}