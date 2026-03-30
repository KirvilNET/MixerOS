@0x91991b41b9b373d1;

using Util = import "../util.capnp";

enum EQType {
  parametric @0;
  filter @1;
}

enum BandTypes {
  # Low pass filter
  lpf @0;

  # High pass filter
  hpf @1;

  # Band pass filter
  bpf @2;

  # Notch filter
  notch @3;

  # Peak filter
  peak @4;

  # Low shelf filter
  lshelf @5;
  
  # High shelf filter
  hshelf @6;
}

struct Band {
  # The id of the band
  id @0 :UInt8;

  # Sampling frequency (hz)
  fs @1 :Float32;

  # Center frequency (hz)
  f0 @2 :Float32;

  # Band parameter Ex. f; bf; s
  parameter @3 :Float32;

  # The kind of band
  kind @4 :BandTypes;

  # Gain
  gain @5 :Float32;

}

struct EQ {
  # Is the EQ module enabled?
  isEnabled @0 :Bool;

  # EQ type
  type @1 :EQType;

  # Max Bands 
  maxBands @2 :UInt8;

  # All the bands of the EQ
  bands @3 :List(Band);
}

interface EQController {
  isEnabled @0 (channelType :Util.ChannelType, id :UInt32) -> (isEnabled :Bool);
  toggle @1 (channelType :Util.ChannelType, id :UInt32, toggle :Bool);

  getCompressorPrameters @2 (channelType :Util.ChannelType, id :UInt32) -> (prams :EQ);
  setCompressorPrameters @3 (channelType :Util.ChannelType, id :UInt32, prams :EQ);
}