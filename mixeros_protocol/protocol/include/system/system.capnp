@0xc735daad7c44666d;
using Util = import "../util.capnp";
using Net = import "./network.capnp";

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
