@0xe00a49a58e05f646;

using Util = import "../util.capnp";

enum GateState {
  closed @0;
  open @1;
}

struct Gate {
  # Is the Gate module enabled?
  isEnabled @0 :Bool;

  # State of the module
  state @1 :GateState;

  # Ammount of gain reduction applied to the module (dB)
  gainReduction @2 :Float32;

  # The threshold of the gate
  threshold @3 :Float32;

  # The attack of the gate (ms)
  attack @4 :Float32;

  # The release of the gate (ms)
  release @5 :Float32;

  # The lookahead delay for the gate (ms)
  lookaheadDelay @6 :Float32;

  # The hold for the gate
  hold @7 :Float32;

  # The floor level for the gate
  floorLevel @8 :Float32;

  # Hysteresis 
  hysteresis @9 :Float32;

  # The maximum attenuation of the gate while closed
  gateRange @10 :Float32;

  # The channel to sidechain this module to (by channel id)
  sidechainFilter @11 :Bool;

}

interface GateController {
  isEnabled @0 (channelType :Util.ChannelType, id :UInt32) -> (isEnabled :Bool);
  toggle @1 (channelType :Util.ChannelType, id :UInt32, toggle :Bool);

  getGatePrameters @2 (channelType :Util.ChannelType, id :UInt32) -> (prams :Gate);
  setGatePrameters @3 (channelType :Util.ChannelType, id :UInt32, prams :Gate);
}