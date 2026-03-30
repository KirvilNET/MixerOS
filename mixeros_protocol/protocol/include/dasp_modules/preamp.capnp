@0x8340a4471a0a0c24;

using Util = import "../util.capnp";

# Preamp/Input stage parameters
struct Preamp {

  # Analog Gain
  analogGain @0 :Float32;

  # Digital Gain
  digitalGain @1 :Float32;

  # -20dB PAD toggle
  pad @2 :Bool;

  # Phantom power
  phantomPower @3 :Bool;

  # Invert polarity toggle
  invert @4 :Bool;

  # Set Input Impedience regulations
  inputImpedience @5 :UInt16;

  # Is Cliping?
  isCliping @6 :Bool;

  # Preamp Headroom
  headroom @7 :Float32;
}

interface LimiterController {
  isEnabled @0 (channelType :Util.ChannelType, id :UInt32) -> (isEnabled :Bool);
  toggle @1 (channelType :Util.ChannelType, id :UInt32, toggle :Bool);

  getPreampPrameters @2 (channelType :Util.ChannelType, id :UInt32) -> (prams :Preamp);
  setPreampPrameters @3 (channelType :Util.ChannelType, id :UInt32, prams :Preamp);
}