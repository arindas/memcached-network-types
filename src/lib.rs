#![doc = include_str!("../README.md")]
#![no_std]

pub mod binary;
pub mod udp;

/// Creates an enum with integer variants.
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

/// Creates constants from [integer_enum] variants.
#[macro_export]
macro_rules! integer_enum_variant_constants {
    ($enum_name:ident, $type:ty, $(($constant_name:ident, $variant_name:ident)),*) => {
        $(
            const $constant_name: $type = $enum_name::$variant_name as $type;
        )*
    };
}
