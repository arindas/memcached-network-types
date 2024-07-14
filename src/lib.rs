#![doc = include_str!("../README.md")]
#![no_std]

pub mod binary;
pub mod udp;

#[doc(hidden)]
#[macro_export]
macro_rules! integer_enum {
    ($enum:ident, $repr:ty, { $( $variant:ident = $value:expr ),* $(,)? }) => {
        #[repr($repr)]
        #[derive(zerocopy_derive::AsBytes, Ord, PartialOrd, Eq, PartialEq, Clone, Copy)]
        pub enum $enum {
            $( $variant = $value ),*
        }

        impl core::convert::TryFrom<$repr> for $enum {
            type Error = $repr;

            #[inline(always)]
            fn try_from(value: $repr) -> Result<Self, Self::Error> {
                match value {
                    $( $value => Ok($enum::$variant), )*
                    _ => Err(value),
                }
            }
        }
    };
}
