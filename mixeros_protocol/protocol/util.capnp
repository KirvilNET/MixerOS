@0xca1b0eed06ce7d69;

enum DeviceType {
    web @0;
    stagebox @1;
    console @2;
}

enum DeviceStatus {
    idle @0;
    online @1;
    offline @2;
    fault @3;
}

enum ProcessorType {
    gpu @0;
    cpu @1;
    npu @2;
    dsp @3;
    fpga @4;
    tpu @5;
    quantum @6;
}

struct Version {
    major @0 :UInt8;
    minor @1 :UInt8;
    patch @2 :UInt8;
}