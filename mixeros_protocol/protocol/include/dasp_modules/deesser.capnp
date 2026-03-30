@0x8816c285c07372eb;

using Util = import "../util.capnp";

enum DeEsserMode {
  deEss @0;
  deBoom @1;
}

struct DeEsser {
  # Is the DeEsser module enabled
  isEnabled @0 :Bool;

  # Suppression band listen toggle
  supressionBandListen @1 :Bool;

  # Filter frequency (Hz)
  filterFreq @2 :Float32;

  # Filter Q 
  filterQ @3 :Float32;

  # Threshold level (dB)
  thresholdLevel @4 :Float32;

  # Filter Range (dB)
  filterRange @5 :Float32;
}

interface DeEsserController {
  isEnabled @0 (channelType :Util.ChannelType, id :UInt32) -> (isEnabled :Bool);
  toggle @1 (channelType :Util.ChannelType, id :UInt32, toggle :Bool);

  getDeEsserPrameters @2 (channelType :Util.ChannelType, id :UInt32) -> (prams :DeEsser);
  setDeEsserPrameters @3 (channelType :Util.ChannelType, id :UInt32, prams :DeEsser);
}