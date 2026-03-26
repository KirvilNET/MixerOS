@0xc735daad7c44666d;
using Util = import "./util.capnp";
using Net = import "./network.capnp";



struct BufferErrors {
    overrun @0 :UInt16;
    underrun @1 :UInt16;
    lastEvent @2 :Int64;
    eventType @3 :EventType;

    enum EventType {
        overrun @0;
        underrun @1;
    }
}

struct CPU {
    name @0 :Text;
    cores @1 :UInt8;
    threads @2 :UInt16;
    temp @3 :Float32;
    usage @4 :List(Float32);
    timestamp @5 :Int64;
}

struct Memory {
    memoryTotal @0 :UInt32;
    memoryUsed @1 :UInt32;
    heapTotal @2 :UInt32;
    heapUsed @3 :UInt32;
}



enum ClockSource {
    local @0;
    worldClock @1;
    network @2;
}

struct DSPProcessor {
    name @0 :Text;
    processorType @1 :Util.ProcessorType;
    status @2 :Util.DeviceStatus;
    temp @3 :Float32;
    load @4 :List(Float32);
    bufferErrors @5 :BufferErrors;
    bufferSize @6 :UInt16;
    moduleCount @7 :UInt32;
    moduleUsage @8 :Float32;
}

struct System {
    cpu @0 :CPU;
    processor @1 :DSPProcessor;
    memory @2 :Memory;
    network @3 :List(Net.Network);
    uptime @4 :UInt64;
    clockSource @5 :ClockSource;
    clockLock @6 :Bool;
}

