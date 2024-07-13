<p align="center">
<h1 align="center"><code>memcached-network-types</code></h1>
</p>

<p align="center">
  <a href="https://github.com/arindas/memcached-network-types/actions/workflows/rust-ci.yml">
    <img src="https://github.com/arindas/memcached-network-types/actions/workflows/rust-ci.yml/badge.svg">
  </a>
  <a href="https://github.com/arindas/memcached-network-types/actions/workflows/rustdoc.yml">
    <img src="https://github.com/arindas/memcached-network-types/actions/workflows/rustdoc.yml/badge.svg">
  </a>
</p>

<p align="center">
Provides types for memcached protocol entities used for sending requests and responses over the network.
</p>

## Usage

`memcached-network-types` is a `nostd` library crate. You may include it in your `Cargo.toml` as follows:

```toml
[dependencies]
memcached-network-types = { git = "https://github.com/arindas/memcached-network-types.git" }
```

Refer to latest git [API Documentation](https://arindas.github.io/memcached-network-types/docs/memcached_network_types/)
for more details.

### Example

```rust
use memcached_network_types::prelude::*;

let req_get_packet_header = ReqPacketHeader {
    magic_byte: ReqMagicByte::ReqPacket,
    opcode: Opcode::Get,
    key_length: 0,
    extras_length: 0,
    data_type: DataType::RawBytes,
    vbucket: 0,
    total_body_length: 0,
    opaque: [0; 4],
    cas: [0; 8],
};

let bytes = req_get_packet_header.as_bytes();

let req_get_packet_header_parsed = ReqPacketHeader::ref_from(bytes).unwrap();

assert!(&req_get_packet_header == req_get_packet_header_parsed);

const GET_OPCODE: u8 = Opcode::Get as u8;

let req_get_packet_header_parsed = unsafe {
    ReqPacketHeader::ref_req_packet_header_with_opcode_from::<GET_OPCODE>(bytes).unwrap()
};

assert!(&req_get_packet_header == req_get_packet_header_parsed);

let req_get_packet_header_parsed =
    ReqPacketHeader::ref_req_packet_header_with_get_opcode_from(bytes).unwrap();

assert!(&req_get_packet_header == req_get_packet_header_parsed);
```

## License

This repository is licensed under the MIT License. See
[LICENSE](https://raw.githubusercontent.com/arindas/memcached-network-types/main/LICENSE) for more details.
