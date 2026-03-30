@0xbe84a981fd123ebc;

using Util = import "../util.capnp";

struct BusDefinition {
  # Internal id for the bus
  id @0 :UInt32;

  # Permissions for the bus
  permissions @1 :Util.Permissions;

  # The type of bus
  busType @2 :Util.BusType;

  # "Physical"/Logical bus number
  busNumber @3 :UInt32;

  # Name of the bus
  busName @4 :Text;

  # Image string for the LCDs on above fader
  busImg @5 :Text;

  # The layer that this bus is assigned to
  layer @6 :UInt16;

  # What channels/buses are assigned to this bus
  input @7 :Util.Port;

  # Virtual Direct Out
  output @8 :Util.Port;

  # matrix assignments
  matrix @9 :List(UInt16);

  # DCA assignments
  dcas @10 :List(UInt16);

  # Mute group assignments
  muteGroups @11 :List(UInt8);
}

struct BusData {
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

}
