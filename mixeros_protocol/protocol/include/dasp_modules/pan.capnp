@0x805a022febbba18b;

using Util = import "../util.capnp";

struct SpeakerDefiniton(Vec) {
  # Id of the speaker in the pan 
  id @0 :UInt32;

  # Name of the speaker
  name @1 :Text;

  # Position of the speaker
  position @2 :Vec;

  # The bus of this speaker
  bus @3 :UInt32;
}

struct Pan {
  # Is the pan module enabled?
  isEnabled @0 :Bool;

  # Speaker definitions for the pan space
  speakerDefinitions @1 :List(SpeakerDefiniton);

  # Pan definiton
  union {
    # Normal Left Right Paning 
    normal :group {
      # The right-left pan (-100-100)
      panValue @2 :Int8;
    }
    # Pan a sound in 2D space
    xy :group {
      # Where the channel is put in 2D space
      sourcePosition @3 :Util.Vec2;

      # The Low Frequency Effect
      lfe @4 :Float32;

      # Slope of the pan
      slope @5 :Float32;
    }
    # Pan a sound in 3D space for souround sound
    pan3D :group {
      # Where the channel is put in 2D space
      sourcePosition @6 :Util.Vec3;

      # The Low Frequency Effect
      lfe @7 :Float32;

      # Slope of the pan
      slope @8 :Float32;

      # Front Width of the source
      frontWidth @9 :Float32;

      # Back Width of the source
      backWidth @10 :Float32;

      # Depth of the source
      depth @11 :Float32;

      # Turn of the source
      turn @12 :Float32;
    }
  }
  
}

interface PanController {
  isEnabled @0 (channelType :Util.ChannelType, id :UInt32) -> (isEnabled :Bool);
  toggle @1 (channelType :Util.ChannelType, id :UInt32, toggle :Bool);

  getPanPrameters @2 (channelType :Util.ChannelType, id :UInt32) -> (prams :Pan);
  setPanPrameters @3 (channelType :Util.ChannelType, id :UInt32, prams :Pan);
}