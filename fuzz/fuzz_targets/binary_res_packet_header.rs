#![no_main]

extern crate memcached_network_types;

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = memcached_network_types::binary::ResPacketHeader::ref_from(data);
});
