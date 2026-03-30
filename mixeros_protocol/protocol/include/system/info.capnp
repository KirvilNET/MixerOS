@0xb7cc47a7948f78e5;
using Util = import "../util.capnp";

struct Buses {
    group @0 :UInt32;
    aux @1 :UInt32;
    sum @2 :UInt32;
}

interface DeviceInfo {

    # Get the name of the device
    getFriendlyName @0 () -> (name :Text);

    # Get the hostname of the device
    getHostname @1 () -> (hostname :Text);

    # Get the MixerOS version of the device
    getVersion @2 () -> (version :Util.Version);

    # Get the id of the device (returns 0 if none is set)
    getId @3 () -> (id :UInt8);

    # Get the device Type
    getDeviceType @4 () -> (deviceType :Util.DeviceType);

    # Get the status of the device
    getStatus @5 () -> (status :Util.DeviceStatus);
    
}

interface DSPInfo {
    # Get the number of inputs
    getNumInput @0 () -> (inputs :UInt32);

    # Get the number of Buses (Groups, Auxes, Sums)
    getBuses @1 () -> (buses :Buses);

    # Get the Main bus and its configuration
    getmainBus @2 () -> (mainBus :UInt16);

    # get the Number of FX (VST3 plugins)
    getNumFX @3 () -> (fx :UInt32);

    # Get the number of DCAs
    getNumdca @4 () -> (dca :UInt32);

    # get the number of mute groups
    getNumMuteGroups @5 () -> (mutegroups :UInt32);

    # Get the sample rate
    getsampleRate @6 () -> (sampleRate :UInt32);

    # Get the desired bit depth
    getbitDepth @7 () -> (bitDepth :UInt8);

    # Get the processor type
    getprocessorType @8 () -> (processorType :Util.ProcessorType);
}

