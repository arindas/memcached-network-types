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
#[derive(FromBytes, FromZeroes, AsBytes, PartialEq, Eq, PartialOrd, Ord)]
pub struct PacketHeader {
    pub magic_byte: u8,
    pub opcode: u8,
    pub key_length: [u8; 2],
    pub extras_length: u8,
    pub data_type: u8,
    pub status_or_vbucket: [u8; 2],
    pub total_body_length: [u8; 4],
    pub opaque: [u8; 4],
    pub cas: [u8; 8],
}

#[repr(C)]
#[derive(AsBytes, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReqPacketHeader {
    pub magic_byte: ReqMagicByte,
    pub opcode: Opcode,
    pub key_length: u16,
    pub extras_length: u8,
    pub data_type: DataType,
    pub vbucket: u16,
    pub total_body_length: u32,
    pub opaque: [u8; 4],
    pub cas: [u8; 8],
}

impl ReqPacketHeader {
    pub fn ref_from(bytes: &[u8]) -> Option<&Self> {
        PacketHeader::ref_from(bytes).and_then(|packet_header| {
            match (
                ReqMagicByte::try_from(packet_header.magic_byte),
                Opcode::try_from(packet_header.opcode),
                DataType::try_from(packet_header.data_type),
            ) {
                (Ok(_), Ok(_), Ok(_)) => Some(unsafe {
                    core::mem::transmute::<&PacketHeader, &ReqPacketHeader>(packet_header)
                }),
                _ => None,
            }
        })
    }

    /// Transumtes a [PacketHeader] read from the given bytes to a [ReqPacketHeader].
    ///
    /// This function ensures that the parsed packet header has [ReqMagicByte::ReqPacket] as its
    /// magic byte and `OPCODE` as its opcode.
    ///
    /// # Safety
    /// `OPCODE` needs to be a valid [Opcode] variant cast to `u8`
    pub unsafe fn ref_req_packet_header_with_opcode_from<const OPCODE: u8>(
        bytes: &[u8],
    ) -> Option<&Self> {
        const REQ_PACKET_MAGIC_BYTE: u8 = ReqMagicByte::ReqPacket as u8;
        const DATA_TYPE_BYTE: u8 = DataType::RawBytes as u8;

        PacketHeader::ref_from(bytes).and_then(|packet_header| {
            match (
                packet_header.magic_byte,
                packet_header.opcode,
                packet_header.data_type,
            ) {
                (REQ_PACKET_MAGIC_BYTE, opcode, DATA_TYPE_BYTE) if opcode == OPCODE => Some(
                    core::mem::transmute::<&PacketHeader, &ReqPacketHeader>(packet_header),
                ),
                _ => None,
            }
        })
    }

    pub fn req_get_packet_header_with_possible_opcode_from<'a>(
        bytes: &'a [u8],
        possible_opcodes: &[Opcode],
    ) -> Option<&'a Self> {
        const REQ_PACKET_MAGIC_BYTE: u8 = ReqMagicByte::ReqPacket as u8;
        const DATA_TYPE_BYTE: u8 = DataType::RawBytes as u8;

        PacketHeader::ref_from(bytes).and_then(|packet_header| {
            match (
                packet_header.magic_byte,
                possible_opcodes
                    .iter()
                    .map(|x| *x as u8)
                    .position(|x| x == packet_header.opcode),
                packet_header.data_type,
            ) {
                (REQ_PACKET_MAGIC_BYTE, Some(_), DATA_TYPE_BYTE) => {
                    Some(unsafe { core::mem::transmute::<&PacketHeader, &Self>(packet_header) })
                }
                _ => None,
            }
        })
    }

    pub fn ref_req_packet_header_with_get_opcode_from(bytes: &[u8]) -> Option<&Self> {
        const GET_OPCODE: u8 = Opcode::Get as u8;

        // SAFETY: GET_OPCODE is Opcode::Get, a valid Opcode variant
        unsafe { Self::ref_req_packet_header_with_opcode_from::<GET_OPCODE>(bytes) }
    }

    pub fn ref_req_packet_header_with_set_opcode_from(bytes: &[u8]) -> Option<&Self> {
        const SET_OPCODE: u8 = Opcode::Set as u8;

        // SAFETY: SET_OPCODE is Opcode::Set, a valid Opcode variant
        unsafe { Self::ref_req_packet_header_with_opcode_from::<SET_OPCODE>(bytes) }
    }
}

#[repr(C)]
#[derive(AsBytes, PartialEq, Eq, PartialOrd, Ord)]
pub struct ResPacketHeader {
    pub magic_byte: ResMagicByte,
    pub opcode: Opcode,
    pub key_length: u16,
    pub extras_length: u8,
    pub data_type: DataType,
    pub status: ResponseStatus,
    pub total_body_length: u32,
    pub opaque: [u8; 4],
    pub cas: [u8; 8],
}

impl ResPacketHeader {
    pub fn ref_from(bytes: &[u8]) -> Option<&Self> {
        PacketHeader::ref_from(bytes).and_then(|packet_header| {
            match (
                ResMagicByte::try_from(packet_header.magic_byte),
                Opcode::try_from(packet_header.opcode),
                DataType::try_from(packet_header.data_type),
                ResponseStatus::try_from(u16::from_be_bytes(packet_header.status_or_vbucket)),
            ) {
                (Ok(_), Ok(_), Ok(_), Ok(_)) => Some(unsafe {
                    core::mem::transmute::<&PacketHeader, &ResPacketHeader>(packet_header)
                }),
                _ => None,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use zerocopy::AsBytes;

    use super::{DataType, Opcode, ReqPacketHeader, ResPacketHeader, ResponseStatus};

    #[test]
    fn req_header_parse_consistent() {
        let req_get_packet_header = ReqPacketHeader {
            magic_byte: super::ReqMagicByte::ReqPacket,
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
    }

    #[test]
    fn res_header_parse_consistent() {
        let res_packet_header = ResPacketHeader {
            magic_byte: super::ResMagicByte::ResPacket,
            opcode: Opcode::Get,
            key_length: 0,
            extras_length: 0,
            data_type: DataType::RawBytes,
            status: ResponseStatus::NoError,
            total_body_length: 0,
            opaque: [0; 4],
            cas: [0; 8],
        };

        let bytes = res_packet_header.as_bytes();

        let res_packet_header_parsed = ResPacketHeader::ref_from(bytes).unwrap();

        assert!(&res_packet_header == res_packet_header_parsed);
    }
}
