@0xc735daad7c44666d;
using Util = import "../util.capnp";
using Net = import "./network.capnp";

struct CPU {
    name @0 :Text;
    vendor @1 :Text;
    vendorId @2 :Text;
    cores @3 :UInt8;
    threads @4 :UInt16;
    temp @5 :Float32;
    usage @6 :Float32;
    timestamp @7 :Int64;
}

struct Memory {
    memoryTotal @0 :UInt32;
    memoryUsed @1 :UInt32;
    heapTotal @2 :UInt32;
    heapUsed @3 :UInt32;
    timestamp @4 :Int64;
}

enum ClockSource {
    local @0;
    worldClock @1;
    network @2;
}

struct Clock {
    source @0 :ClockSource;
    port @1 :UInt16;
    union { 
        ipv4 @2 :Net.IPv4; 
        ipv6 @3 :Net.IPv6; 
    }
}

interface System {
    getCpu @0 () -> (data: List(CPU));
    getMemory @1 () -> (data: Memory);
    getNetwork @2 () -> (data: List(Net.Interface));
    getClock @3 () -> (data: Clock);
}