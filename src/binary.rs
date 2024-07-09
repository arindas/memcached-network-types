//! Provides types pertaining to Memcached binary [protocol](https://github.com/memcached/memcached/wiki/BinaryProtocolRevamped).

use zerocopy::FromBytes;
use zerocopy_derive::{AsBytes, FromBytes, FromZeroes};

use crate::integer_enum;

integer_enum! {
    ReqMagicByte, u8, {
        ReqPacket = 0x80,
    }
}

integer_enum! {
    ResMagicByte, u8, {
        ResPacket = 0x81,
    }
}

integer_enum! {
    DataType, u8, {
        RawBytes = 0x00,
    }
}

integer_enum! {
    ResponseStatus, u16, {
        NoError = 0x0000,
        KeyNotFound = 0x0001,
        KeyExists = 0x0002,
        ValueTooLarge = 0x0003,
        InvalidArguments = 0x0004,
        ItemNotStored = 0x0005,
        IncrOrDecrOnNumericValue = 0x0006,
        VBucketBelongsToAnotherServer = 0x0007,
        AuthenticationError = 0x0008,
        AuthenticationContinue = 0x0009,
        UnknownCommand = 0x0081,
        OutOfMemory = 0x0082,
        NotSupported = 0x0083,
        InternalError = 0x0084,
        Busy = 0x0085,
        TemporaryFailure = 0x0086,
    }
}

integer_enum! {
    Opcode, u8, {
        Get = 0x00,
        Set = 0x01,
        Add = 0x02,
        Replace = 0x03,
        Delete = 0x04,
        Increment = 0x05,
        Decrement = 0x06,
        Quit = 0x07,
        Flush = 0x08,
        GetQ = 0x09,
        NoOp = 0x0a,
        Version = 0x0b,
        GetK = 0x0c,
        GetKQ = 0x0d,
        Append = 0x0e,
        Prepend = 0x0f,
        Stat = 0x10,
        SetQ = 0x11,
        AddQ = 0x12,
        ReplaceQ = 0x13,
        DeleteQ = 0x14,
        IncrementQ = 0x15,
        DecrementQ = 0x16,
        QuitQ = 0x17,
        FlushQ = 0x18,
        AppendQ = 0x19,
        PrependQ = 0x1a,
        Verbosity = 0x1b,
        Touch = 0x1c,
        Gat = 0x1d,
        Gatq = 0x1e,
        SaslListMechs = 0x20,
        SaslAuth = 0x21,
        SaslStep = 0x22,
        RGet = 0x30,
        RSet = 0x31,
        RSetQ = 0x32,
        RAppend = 0x33,
        RAppendQ = 0x34,
        RPrepend = 0x35,
        RPrependQ = 0x36,
        RDelete = 0x37,
        RDeleteQ = 0x38,
        RIncr = 0x39,
        RIncrQ = 0x3a,
        RDecr = 0x3b,
        RDecrQ = 0x3c,
        SetVBucket = 0x3d,
        GetVBucket = 0x3e,
        DelVBucket = 0x3f,
        TapConnect = 0x40,
        TapMutation = 0x41,
        TapDelete = 0x42,
        TapFlush = 0x43,
        TapOpaque = 0x44,
        TapVBucketSet = 0x45,
        TapCheckpointStart = 0x46,
        TapCheckpointEnd = 0x47,
    }
}

#[repr(C)]
#[derive(FromBytes, FromZeroes, AsBytes)]
pub struct PacketHeader {
    magic_byte: u8,
    opcode: u8,
    key_length: [u8; 2],
    extras_length: u8,
    data_type: u8,
    status_or_vbucket: [u8; 2],
    total_body_length: [u8; 4],
    opaque: [u8; 4],
    cas: [u8; 8],
}

#[repr(C)]
#[derive(AsBytes)]
pub struct ReqPacketHeader {
    magic_byte: ReqMagicByte,
    opcode: Opcode,
    key_length: u16,
    extras_length: u8,
    data_type: u8,
    vbucket: u16,
    total_body_length: u32,
    opaque: [u8; 4],
    cas: [u8; 8],
}

impl ReqPacketHeader {
    pub fn ref_from(bytes: &[u8]) -> Option<&Self> {
        PacketHeader::ref_from(bytes).and_then(|packet_header| {
            match (
                ReqMagicByte::try_from(packet_header.magic_byte),
                Opcode::try_from(packet_header.opcode),
            ) {
                (Ok(_), Ok(_)) => Some(unsafe {
                    core::mem::transmute::<&PacketHeader, &ReqPacketHeader>(packet_header)
                }),
                _ => None,
            }
        })
    }

    pub fn ref_req_packet_header_with_opcode_from<const OPCODE: u8>(bytes: &[u8]) -> Option<&Self> {
        const REQ_PACKET_MAGIC_BYTE: u8 = ReqMagicByte::ReqPacket as u8;

        PacketHeader::ref_from(bytes).and_then(|packet_header| {
            match (packet_header.magic_byte, packet_header.opcode) {
                (REQ_PACKET_MAGIC_BYTE, opcode) if opcode == OPCODE => Some(unsafe {
                    core::mem::transmute::<&PacketHeader, &ReqPacketHeader>(packet_header)
                }),
                _ => None,
            }
        })
    }

    pub fn ref_req_packet_header_with_get_opcode_from(bytes: &[u8]) -> Option<&Self> {
        const REQ_PACKET_MAGIC_BYTE: u8 = ReqMagicByte::ReqPacket as u8;
        const GET_OPCODE: u8 = Opcode::Get as u8;

        PacketHeader::ref_from(bytes).and_then(|packet_header| {
            match (packet_header.magic_byte, packet_header.opcode) {
                (REQ_PACKET_MAGIC_BYTE, GET_OPCODE) => Some(unsafe {
                    core::mem::transmute::<&PacketHeader, &ReqPacketHeader>(packet_header)
                }),
                _ => None,
            }
        })
    }

    pub fn ref_req_packet_header_with_set_opcode_from(bytes: &[u8]) -> Option<&Self> {
        const REQ_PACKET_MAGIC_BYTE: u8 = ReqMagicByte::ReqPacket as u8;
        const SET_OPCODE: u8 = Opcode::Set as u8;

        PacketHeader::ref_from(bytes).and_then(|packet_header| {
            match (packet_header.magic_byte, packet_header.opcode) {
                (REQ_PACKET_MAGIC_BYTE, SET_OPCODE) => Some(unsafe {
                    core::mem::transmute::<&PacketHeader, &ReqPacketHeader>(packet_header)
                }),
                _ => None,
            }
        })
    }
}

#[repr(C)]
#[derive(AsBytes)]
pub struct ResPacketHeader {
    magic_byte: ResMagicByte,
    opcode: Opcode,
    key_length: u16,
    extras_length: u8,
    data_type: u8,
    status: ResponseStatus,
    total_body_length: u32,
    opaque: [u8; 4],
    cas: [u8; 8],
}

impl ResPacketHeader {
    pub fn ref_from(bytes: &[u8]) -> Option<&Self> {
        PacketHeader::ref_from(bytes).and_then(|packet_header| {
            match (
                ResMagicByte::try_from(packet_header.magic_byte),
                Opcode::try_from(packet_header.opcode),
                ResponseStatus::try_from(u16::from_be_bytes(packet_header.status_or_vbucket)),
            ) {
                (Ok(_), Ok(_), Ok(_)) => Some(unsafe {
                    core::mem::transmute::<&PacketHeader, &ResPacketHeader>(packet_header)
                }),
                _ => None,
            }
        })
    }
}
