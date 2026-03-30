@0xd236b0372b3751ac;

using Util = import "util.capnp";
using Info = import "system/info.capnp";

interface Device {
  getDeviceType @0 () -> (deviceType :Util.DeviceType);
  getName @1 () -> (name :Text);
  setName @2 (name :Text);

  getStatus @3 () -> (status :Util.DeviceType);

  getDeviceInfo @4 () -> (info :Info.DeviceInfo);
}

