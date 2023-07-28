use semver::Version;

macro_rules! flat_mod {
    ($($i:ident),+) => {
        $(
            mod $i;
            pub use $i::*;
        )+
    };
}

pub mod compiler;
pub mod error;
pub mod parsed_ir;
#[path = "bindings.rs"]
pub mod sys;

flat_mod! {
    context
}

#[doc(inline)]
pub use error::Error;
#[doc(inline)]
pub use parsed_ir::ParsedIr;

pub const SPVC_VERSION: Version = Version::new(
    sys::SPVC_C_API_VERSION_MAJOR as u64,
    sys::SPVC_C_API_VERSION_MINOR as u64,
    sys::SPVC_C_API_VERSION_PATCH as u64,
);
