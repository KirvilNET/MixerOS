pub mod mixeros_protocol_sys {
  include!(concat!(env!("OUT_DIR"), "/genarated_files.rs"));
}

// include/dasp_modules
pub(crate) use mixeros_protocol_sys::proto::include::dasp_modules::compressor_capnp;
pub(crate) use mixeros_protocol_sys::proto::include::dasp_modules::deesser_capnp;
pub(crate) use mixeros_protocol_sys::proto::include::dasp_modules::eq_capnp;
pub(crate) use mixeros_protocol_sys::proto::include::dasp_modules::expander_capnp;
pub(crate) use mixeros_protocol_sys::proto::include::dasp_modules::gate_capnp;
pub(crate) use mixeros_protocol_sys::proto::include::dasp_modules::image_capnp;
pub(crate) use mixeros_protocol_sys::proto::include::dasp_modules::limiter_capnp;
pub(crate) use mixeros_protocol_sys::proto::include::dasp_modules::pan_capnp;
pub(crate) use mixeros_protocol_sys::proto::include::dasp_modules::preamp_capnp;

// include
pub(crate) use mixeros_protocol_sys::proto::include::device_capnp;
pub(crate) use mixeros_protocol_sys::proto::include::util_capnp;

// include/system
pub(crate) use mixeros_protocol_sys::proto::include::system::info_capnp;
pub(crate) use mixeros_protocol_sys::proto::include::system::network_capnp;
pub(crate) use mixeros_protocol_sys::proto::include::system::system_capnp;
//pub use mixeros_protocol_sys::proto::include::system::scenes_capnp;

// include/sources/control
pub(crate) use mixeros_protocol_sys::proto::include::sources::control::dca_capnp;
pub(crate) use mixeros_protocol_sys::proto::include::sources::control::mutegroup_capnp;

// include/sources
pub(crate) use mixeros_protocol_sys::proto::include::sources::bus_capnp;
pub(crate) use mixeros_protocol_sys::proto::include::sources::channel_capnp;
pub(crate) use mixeros_protocol_sys::proto::include::sources::mains_capnp;

pub(crate) use mixeros_protocol_sys::proto::engine_capnp;