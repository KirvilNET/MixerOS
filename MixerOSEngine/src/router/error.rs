#[derive(Debug)]
pub enum ChannelStripError {
  InvalidName,
  InvalidLevel,
  InvalidGain,
  InvalidInput,
  InvalidOutput,
  NoInput,
  InputError,
  StreamError,
  SystemChannel
}

#[derive(Debug)]
pub enum BusError {
  InvalidName,
  InvalidLevel,
  InvalidGain,
  InvalidInput,
  InvalidOutput,
  NoOutput,
}