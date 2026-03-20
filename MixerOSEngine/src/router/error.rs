#[derive(Debug)]
pub enum ChannelStripError {
  InvalidName,
  InvalidLevel,
  InvalidGain,
  InvalidInput,
  InvalidOutput,
  NoInput,
  InputError,
  NotInitilized,
  SystemChannel
}

#[derive(Debug)]
pub enum BusError {
  InvalidName,
  InvalidLevel,
  InvalidGain,
  InvalidInput,
  InvalidOutput,
  NotInitilized,
  NoOutput,
  SystemChannel
}