//! Contains types related to memcached [UDP protocol](https://github.com/memcached/memcached/blob/b1aefcdf8a265f8a5126e8aa107a50988fa1ec35/doc/protocol.txt#L1861).

use zerocopy::network_endian::U16;
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

#[derive(FromBytes, FromZeroes, AsBytes)]
#[repr(C)]
pub struct MemcachedUdpHeader {
    pub request_id: U16,
    pub seq_num: U16,
    pub num_dgram: U16,
    pub unused: U16,
}
