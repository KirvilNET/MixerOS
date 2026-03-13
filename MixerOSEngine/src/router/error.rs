#[derive(Debug)]
pub enum ChannelStripError {
  InvalidName,
  InvalidLevel,
  InvalidGain,
  InvalidInput,
  InvalidOutput,
  StreamError
}

#[derive(Debug)]
pub enum BusError {
  InvalidName,
  InvalidLevel,
  InvalidGain,
  InvalidInput,
  InvalidOutput,
}