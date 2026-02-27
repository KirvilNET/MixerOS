#[derive(Debug)]

pub enum ChannelStripError {
  INVALID_NAME,
  INVALID_LEVEL,
  INVALID_GAIN,
  INVALID_INPUT,
  INVALID_OUTPUT
}