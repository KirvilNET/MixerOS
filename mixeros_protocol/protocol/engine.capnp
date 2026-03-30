@0xfadf92212b998c0d;

using Util = import "include/util.capnp";
using Channel = import "include/sources/channel.capnp";
using Bus = import "include/sources/bus.capnp";
using Info = import "include/system/info.capnp";
using Net = import "include/system/network.capnp";
using System = import "include/system/system.capnp";
using Device = import "include/device.capnp";

enum EngineRole {

  # Defines the engine as a cluster controller.
  controller @0;

  # Defines the engine as a node in a cluster.
  node @1;

  # Defines the engine as a redundancy node.
  # Can either be a Redundant Controller or Node.
  redundancyNode @2;
}

interface Engine {
  getRole @0 () -> (role :EngineRole);
  
}

interface DASPInfo extends(Engine) {

  #? Audio Sources and Outputs

  # Get the number of channels
  getNumChannels @0 () -> (channels :UInt32);

  # Get the number of aux buses
  getNumAuxes @1 () -> (auxes :UInt32);

  # Get the number of matrix buses
  getNumMatrices @2 () -> (matrices :UInt32);

  # Get the number of groups
  getNumGroups @3 () -> (groups :UInt32);

  #? Audio Controls

  # Get the number of DCAs 
  getNumDca @4 () -> (dca :UInt32);

  # Get the number of Mute Groups
  getNumMuteGroups @5 () -> (mutegroups :UInt32);

  #? System Status

  # Get cpu data
  getCPU @6 () -> (cpu :System.CPU);

  # Get cpu data
  getMemory @7 () -> (memory :System.Memory);

  # Get cpu data
  getProcessor @8 () -> (processor :System.CPU);

  # Get cpu data
  getUptime @9 () -> (uptime :UInt64);

  # Get cpu data
  getClockData @10 () -> (clockSource :System.ClockSource, isLocked :Bool);

  # Get network data
  getNetwork @11 () -> (interfaces :List(Net.Interface));

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