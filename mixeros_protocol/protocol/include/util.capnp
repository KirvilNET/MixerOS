@0xca1b0eed06ce7d69;

# Vector 1 Data
struct Vec1(DataType) {

}

# Vector 2 Data
struct Vec2(DataType) {
    x @0 :DataType;
    y @1 :DataType;
}

# Vector 3 Data
struct Vec3(DataType) {
    x @0 :DataType;
    y @1 :DataType;
    z @2 :DataType;
}

enum KneeType {
  soft @0;
  hard @1;
}

enum ChannelType {
    bus @0;
    channel @1;
}

enum BusType {
  group @0;
  aux @1;
  matrix @2;
}

struct Port {
  name @0 :Text;
  id @1 :UInt64;
}

enum MuteSource {
  manual @0;
  dca @1;
  muteGroup @2;
  scene @3;
  automation @4;
}

enum SoloType {
  pfl @0;
  afl @1;
  iso @2;
}

struct Fader {
  # Current fader level (dB)
  realLevel @0 :Float32;

  # The intended level (dB) 
  # What the software is trying to set the motorized fader to
  level @1 :Float32;

  # Is touched (Apparently some faders have touch sensing)
  isTouched @2 :Bool;

  # Is the fader currently moving
  isMoving @3 :Bool;
}

enum DeviceType {
    # Web UI (Engine Config UI, Console WebUI, Mobile App)
    web @0;

    # StageBoxes
    stagebox @1;

    # Mixing Console 
    console @2;

    # Processing Engine
    engine @3;
}

enum DeviceStatus {
    idle @0;
    online @1;
    offline @2;
    fault @3;
}

enum ProcessorType {

    # GPU processing 
    gpu @0;

    # CPU with SIMD perfered
    cpu @1;

    # Neural Processing Unit
    npu @2;

    # Digital Signal Processing chip
    dsp @3;

    # FPGA processing units
    fpga @4;

    # Tensor processing units
    tpu @5;

    # Quantum processing unit (cause why not)
    quantum @6;
}

struct OpenCLDevice {
    deviceName @0 :Text;
    deviceVendor @1 :Text;
    deviceVersion @2 :Text;
    driverVersion @3 :List(Text);
    deviceType @4 :ProcessorType;
    isAvalible @5 :Bool;
}

struct Processor {
    platformName @0 :Text;
    platformVendor @1 :Text;
    devices @2 :List(OpenCLDevice);
}

struct Version {
    major @0 :UInt8;
    minor @1 :UInt8;
    patch @2 :UInt8;
}

enum Permissions {
  # Channel/Bus that cannot be changed by the user
  system @0;

  # Channel/Bus that can be changed by the user
  user @1;
}