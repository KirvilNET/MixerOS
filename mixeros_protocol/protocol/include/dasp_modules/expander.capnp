@0xfe48d2da24c7f324;

using Util = import "../util.capnp";

enum ExpanderState {
  closed @0;
  open @1;
}

struct Expander {
  # Is the expander module enabled?
  isEnabled @0 :Bool;

  # State of the module
  state @1 :ExpanderState;

  # Ammount of gain reduction applied to the module (dB)
  gainReduction @2 :Float32;

  # The threshold of the expander
  threshold @3 :Float32;

  # The attack of the expander (ms)
  attack @4 :Float32;

  # The release of the expander (ms)
  release @5 :Float32;

  # The lookahead delay for the expander (ms)
  lookaheadDelay @6 :Float32;

  # The maximum attenuation of the expander while closed
  expanderRange @7 :Float32;

  # the ratio for the expander
  ratio @8 :Float32;

  # The floor level for the expander
  floorLevel @9 :Float32;

  # The channel to sidechain this module to (by channel id)
  sidechainChannel @10 :UInt32;

}

interface ExpanderController {
  isEnabled @0 (channelType :Util.ChannelType, id :UInt32) -> (isEnabled :Bool);
  toggle @1 (channelType :Util.ChannelType, id :UInt32, toggle :Bool);

  getExpanderPrameters @2 (channelType :Util.ChannelType, id :UInt32) -> (prams :Expander);
  setExpanderPrameters @3 (channelType :Util.ChannelType, id :UInt32, prams :Expander);
}