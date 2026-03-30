@0xa21ad2ac67778f15;

using Util = import "../util.capnp";

struct Compressor {
  # Is the Compressor module enabled?
  isEnabled @0 :Bool;

  # The attack of the compressor (ms)
  attack @1 :Float32;

  # The release of the compressor (ms)
  release @2 :Float32;

  # The threshold of the compressor (dB)
  threshold @3 :Float32;

  # The lookahead delay for the expander (ms)
  lookaheadDelay @4 :Float32;

  # The ratio of the compressor Ex. 0.5 = 1/2
  ratio @5 :Float32;

  # Ammount of makeup gain applied to the module (dB)
  makeupGain @6 :Float32;

  # the mix level for paralell compression (%)
  mixLevel @7 :Float32;

  # The knee type
  kneeType @8 :Util.KneeType;

  # Toggle the sidechain filter
  sidechainFilter @9 :Bool;

  # Enable/disable auto makeup gain
  autoMakeup @10 :Bool;
}

interface CompressorController {
  isEnabled @0 (channelType :Util.ChannelType, id :UInt32) -> (isEnabled :Bool);
  toggle @1 (channelType :Util.ChannelType, id :UInt32, toggle :Bool);

  getCompressorPrameters @2 (channelType :Util.ChannelType, id :UInt32) -> (prams :Compressor);
  setCompressorPrameters @3 (channelType :Util.ChannelType, id :UInt32, prams :Compressor);
}