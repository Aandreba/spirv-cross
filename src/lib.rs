use semver::Version;
use std::borrow::Cow;

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

/// Raw bindings to the C
#[path = "bindings.rs"]
pub mod sys;

flat_mod! {
    context
}

#[doc(inline)]
pub use error::Error;

pub const SPVC_VERSION: Version = Version::new(
    sys::SPVC_C_API_VERSION_MAJOR as u64,
    sys::SPVC_C_API_VERSION_MINOR as u64,
    sys::SPVC_C_API_VERSION_PATCH as u64,
);

/// Converts a byte slice into a word slice, avoiding a new allocation if possible.
///
/// Returns `None` if the amount of bytes doesn't result into a whole number of words.
pub fn bytes_to_words(bytes: &[u8]) -> Option<Cow<'_, [u32]>> {
    const SIZE: usize = core::mem::size_of::<u32>();

    if bytes.len() % SIZE != 0 {
        return None;
    }

    let word_count = bytes.len() / SIZE;
    return Some(
        match bytes.as_ptr().align_offset(core::mem::align_of::<u32>()) {
            0 => unsafe {
                Cow::Borrowed(core::slice::from_raw_parts(
                    bytes.as_ptr().cast(),
                    word_count,
                ))
            },

            _ => {
                let mut words = Vec::<u32>::with_capacity(word_count);
                unsafe {
                    core::ptr::copy_nonoverlapping(
                        bytes.as_ptr(),
                        words.as_mut_ptr().cast(),
                        SIZE * word_count,
                    );
                    words.set_len(bytes.len() / SIZE);
                }
                Cow::Owned(words)
            }
        },
    );
}
