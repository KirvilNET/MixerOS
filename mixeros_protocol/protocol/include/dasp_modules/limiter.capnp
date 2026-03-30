@0x9752fa01ff0b0fa0;

using Util = import "../util.capnp";

struct Limiter {
  # Is the limiter module enabled?
  isEnabled @0 :Bool;

  # The Attack time of the limiter (ms)
  attack @1 :Float32;

  # The Release time of the limiter (ms)
  release @2 :Float32;

  # The Threshold level of the limiter (dB)
  threshold @3 :Float32;

  # The lookahead delay for the limiter (ms)
  lookaheadDelay @4 :Float32;

  # The hold time for the limiter (ms)
  holdTime @5 :Float32;

  # The knee type of the limiter 
  knee @6 :Util.KneeType;

  # Toggle the sidechain filter
  sidechainFilter @7 :Bool;
}

interface LimiterController {
  isEnabled @0 (channelType :Util.ChannelType, id :UInt32) -> (isEnabled :Bool);
  toggle @1 (channelType :Util.ChannelType, id :UInt32, toggle :Bool);

  getLimiterPrameters @2 (channelType :Util.ChannelType, id :UInt32) -> (prams :Limiter);
  setLimiterPrameters @3 (channelType :Util.ChannelType, id :UInt32, prams :Limiter);
}