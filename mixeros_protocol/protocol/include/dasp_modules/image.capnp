@0xfae0ec8414241a2c;

using Util = import "../util.capnp";

struct Image {
  # Is the Image module enabled?
  isEnabled @0 :Bool;

  # Style of the image
  # true = position adjusts the left (or right) of the image
  # false = position adjusts the centre of the image
  style @1 :Bool;

  # Width of the image
  width @2 :Float32;

  # Position of the image
  position @3 :Float32;

}

interface CompressorController {
  isEnabled @0 (channelType :Util.ChannelType, id :UInt32) -> (isEnabled :Bool);
  toggle @1 (channelType :Util.ChannelType, id :UInt32, toggle :Bool);

  getImagePrameters @2 (channelType :Util.ChannelType, id :UInt32) -> (prams :Image);
  setImagePrameters @3 (channelType :Util.ChannelType, id :UInt32, prams :Image);
}