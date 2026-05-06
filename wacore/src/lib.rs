extern crate self as wa_rs_core;

pub use wa_rs_appstate as appstate;
pub use wa_rs_noise as noise;

// Re-export derive macros
pub use wa_rs_derive::{EmptyNode, ProtocolNode, WireEnum};

pub mod adv;
pub mod appstate_sync;
pub mod client;
pub mod client_profile;
pub mod companion_reg;
pub mod download;
pub mod iq;
pub mod protocol;
pub use wa_rs_noise::framing;
pub mod handshake;
pub mod history_sync;
pub mod ib;
pub use wa_rs_libsignal as libsignal;
pub mod media_retry;
pub mod message_processing;
pub mod messages;
pub mod net;
pub mod pair;
pub mod pair_code;
pub mod poll;
pub mod prekeys;
pub mod proto_helpers;
pub mod reporting_token;
pub mod request;
pub mod runtime;
pub mod send;
pub mod session;
pub mod stanza;
pub mod sticker_pack;

pub mod store;
pub mod time;
pub mod types;
pub mod upload;
pub mod usync;
pub mod webp;

pub mod version;
pub mod xml;
mod zip;
