@0xa5717fac0caf0ee2;

using Util = import "../util.capnp";

struct ChannelDefinition {
  # Internal id for the channel
  id @0 :UInt32;

  # Permissions for the channel
  permissions @1 :Util.Permissions;

  # "Physical"/Logical channel number
  channelNumber @2 :UInt32;

  # Name of the channel
  channelName @3 :Text;

  # Image string for the LCDs on the fader
  channelImg @4 :Text;

  # The layer that this channel is assigned to
  layer @5 :UInt16;

  # The Inputs for this channel (2 inputs for hotswap capablities ex. dual micing)
  inputA @6 :Util.Port;
  inputB @7 :Util.Port;

  # Virtual Direct Out
  output @8 :Util.Port;

  # Bus assignments
  buses @9 :List(UInt16);

  # DCA assignments
  dcas @10 :List(UInt16);

  # Mute group assignments
  muteGroups @11 :List(UInt8);
}

struct Send {
  # Is Enabled
  isEnabled @0 :Bool;

  # the level of the send (dB)
  level @1 :Float32;

  # pre/post
  prePost @2 :Bool;
}

struct ChannelData {
  # all of the fader data
  fader @0 :Util.Fader;

  # Mute State
  mute @1 :Bool;

  # Mute Source
  muteSource @2 :Util.MuteSource;

  # Is soloed
  solo @3 :Bool;

  # Solo Type
  soloType @4 :Util.SoloType;

  # The sends of the channel and their data
  sends @5 :List(Send);
}