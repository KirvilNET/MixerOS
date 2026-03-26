@0xb7cc47a7948f78e5;
using Util = import "./util.capnp";

struct DeviceInfo {
    friendlyName @0 :Text;
    hostname @1 :Text;
    version @2 :Util.Version;
    id @3 :UInt8;
    deviceType @4 :Util.DeviceType;
    status @5 :Util.DeviceStatus;
    serial @6 :UInt16;
}

struct DSPInfo {
    input @0 :UInt32;
    auxBuses @1 :UInt32;
    matrix @2 :UInt8;
    mainBus @3 :UInt8;
    fxCapacity @4 :UInt16;
    dca @5 :UInt16;
    sampleRate @6 :UInt32;
    bitDepth @7 :UInt8;
    processorType @8 :Util.ProcessorType;
}

