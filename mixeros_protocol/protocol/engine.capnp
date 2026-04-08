@0xfadf92212b998c0d;

using Util = import "include/util.capnp";
using Channel = import "include/sources/channel.capnp";
using Bus = import "include/sources/bus.capnp";
using Info = import "include/system/info.capnp";
using Net = import "include/system/network.capnp";
using System = import "include/system/system.capnp";
using Device = import "include/device.capnp";

struct NumBuses {
  auxes @0 :UInt32;
  groups @1 :UInt32;
  matrices @2 :UInt32;
}

interface Engine {
  getRole @0 () -> (role :Util.EngineRole);
  setRole @1 (role :Util.EngineRole);
}

interface DASPInfo extends(Engine) {

  #? Audio Sources and Outputs

  # Get the number of channels
  getNumChannels @0 () -> (channels :UInt32);

  # Get the number of buses 
  getNumBuses @1 () -> (buses :NumBuses);

  # Get the number of groups
  getNumGroups @2 () -> (groups :UInt32);

  #? Audio Controls

  # Get the number of DCAs 
  getNumDca @3 () -> (dca :UInt32);

  # Get the number of Mute Groups
  getNumMuteGroups @4 () -> (mutegroups :UInt32);

  #? System Status

  # Get cpu data
  getCPU @5 () -> (cpu :System.CPU);

  # Get cpu data
  getMemory @6 () -> (memory :System.Memory);

  # Get cpu data
  getProcessor @7 () -> (processor :System.CPU);

  # Get cpu data
  getUptime @8 () -> (uptime :UInt64);

  # Get cpu data
  getClockData @9 () -> (clockSource :System.ClockSource, isLocked :Bool);

  # Get network data
  getNetwork @10 () -> (interfaces :List(Net.Interface));

}

interface Channels extends(Engine) {
  #? Channel Mutators

  # Get the the channels
  getChannels @0 () -> (channels :List(Channel.ChannelDefinition));

  # Remove channel by id
  removeChannel @1 (id :UInt32);

  # Create a new channel
  createChannel @2 (channel :Channel.ChannelDefinition);

  #? Fader 

  # Get the current fader level in dB
  getFaderData @3 (id :UInt32) -> (level :Util.Fader);

  # Set the fader at a dB level
  setFaderLevel @4 (id :UInt32, level :Float32);

  #? Channel Config

  # Get the data of the Channel
  getChannelData @5 (id :UInt32) -> (data :Channel.ChannelData);

}

interface Buses extends(Engine) {
  #? Channel Mutators

  # Get the the channels
  getBuses @0 () -> (buses :List(Bus.BusDefinition));

  # Remove channel by id
  removeBus @1 (id :UInt32);

  # Create a new channel
  createBus @2 (channel :Bus.BusDefinition);

  #? Fader 

  # Get the current fader level in dB
  getFaderData @3 (id :UInt32) -> (level :Util.Fader);

  # Set the fader at a dB level
  setFaderLevel @4 (id :UInt32, level :Float32);

  #? Channel Config

  # Get the data of the Channel
  getBusData @5 (id :UInt32) -> (data :Bus.BusData);
  
}